//! Nova's Render Hardware Interface
//!
//! This is an interface to the GPU which has been designed for Nova. It abstracts away parts of the
//! underlying APIs which Nova doesn't use, providing an interface that's more productive and more
//! fun. The RHI is actually split into two sections: the synchronous parts and the asynchronous
//! part. The synchronous part of the API is where your calls happen immediately on the GPU, while
//! the asynchronous part is where your calls get recorded into command lists, which are later
//! executed on the GPU

use std::collections::HashMap;

use super::rhi_enums::*;
use super::rhi_structs::*;

/// Top-level trait for functions that don't belong to any specific device object
pub trait GraphicsApi {
    type PhysicalDevice: PhysicalDevice;

    /// Gets a list of all available graphics adapters
    fn get_adapters() -> Vec<Self::PhysicalDevice>;
}

/// An implementation of the rendering API. This will probably be a GPU card, but a software
/// implementation of either Vulkan or Direct3D 12 is possible
pub trait PhysicalDevice {
    type Device: Device;

    fn get_properties(&self) -> PhysicalDeviceProperties;

    /// Checks if this physical device is suitable for Nova
    ///
    /// Devices are suitable for Nova if they
    /// - Have queues that support graphics, compute, transfer, and present operations
    /// - Support tessellation and geometry shaders
    /// Nova's supported APIs have very different ways to check what features and capabilities a
    ///
    /// physical device has, so this method encapsulates all that
    ///
    /// Future work will probably come up with a way to score physical devices from most suitable to
    /// least suitable, but for now this is fine
    fn can_be_used_by_nova(&self) -> bool;

    /// Creates a new logical Device
    ///
    /// Nova has very specific requirements for a logical device, and how you express those
    /// requirements varies significantly by API. Thus, this method doesn't take a create info
    /// struct of any sort
    fn create_logical_device(&self) -> Result<Self::Device, DeviceCreationError>;
}

/// The logical device that we're rendering with
///
/// There may be multiple Devices in existence at once. Nova will eventually support multi-GPU
/// rendering
pub trait Device {
    type Queue: Queue;
    type Memory: Memory;
    type CommandAllocator: CommandAllocator;
    type Image: Image;
    type Renderpass: Renderpass;
    type Framebuffer: Framebuffer;
    type PipelineInterface: PipelineInterface;
    type DescriptorPool: DescriptorPool;
    type Pipeline: Pipeline;
    type Semaphore: Semaphore;
    type Fence: Fence;

    /// Retrieves the Queue with the provided queue family index and queue index
    ///
    /// The caller should verify that the device supports the requested queue index and queue
    /// family index
    ///
    /// # Parameters
    ///
    /// * `queue_family_index` - The queue family index to get a queue from
    /// * `queue_index` - The index of the queue to get from the selected queue family
    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Result<Self::Queue, QueueGettingError>;

    /// Allocates memory from the graphics API
    ///
    /// This memory may be on the device or on the host, depending on its usage and allowed objects
    ///
    /// # Parameters
    ///
    /// * `size` - The size, in bytes, of the memory you want to allocate
    /// * `memory_usage` - The usage you want the memory to be usable for
    /// * `allowed_objects` - The types of objects you want to allow from this memory. Enforcing
    /// this is up to the caller
    fn allocate_memory<T>(
        &self,
        size: u64,
        memory_usage: MemoryUsage,
        allowed_objects: ObjectType,
    ) -> Result<Self::Memory, AllocationError>;

    /// Creates a new CommandAllocator
    ///
    /// # Parameters
    ///
    /// * `create_info` - Information about how you want the CommandAllocator created
    fn create_command_allocator(
        &self,
        create_info: CommandAllocatorCreateInfo,
    ) -> Result<Self::CommandAllocator, MemoryError>;

    /// Creates a new renderpass from the provided shaderpack data
    ///
    /// # Parameters
    ///
    /// * `data` - The shaderpack data to create the renderpass from
    fn create_renderpass(&self, data: shaderpack::RenderPassCreationInfo) -> Result<Self::Renderpass, MemoryError>;
    fn create_renderpass(&self, data: RenderpassData) -> Result<Self::Renderpass, MemoryError>;

    /// Creates a new Framebuffer
    ///
    /// Framebuffers get their attachment layout from a renderpass. I do not know why Khronos didn't
    /// make a separate type for a framebuffer interface, yet here we are. Thus, this method takes in
    /// the renderpass to use an interface
    ///
    /// # Parameters
    ///
    /// * `renderpass` - The Renderpass to get the framebuffer layout from
    /// * `attachments` - The images to attach to the framebuffer, in attachment order
    /// * `framebuffer_size` - The size of the framebuffer, in pixels
    fn create_framebuffer(
        &self,
        renderpass: Self::Renderpass,
        attachments: Vec<Self::Image>,
        framebuffer_size: Vec2,
    ) -> Result<Self::Framebuffer, MemoryError>;

    /// Creates a PipelineInterface from the provided information
    ///
    /// # Parameters
    ///
    /// * `bindings` - The bindings that the pipeline exposes
    /// * `color_attachments` - All the color attachments that the pipeline writes to
    /// * `depth_texture` - The depth texture that this pipeline writes to, if it writes to one
    fn create_pipeline_interface(
        &self,
        bindings: &HashMap<String, ResourceBindingDescription>,
        color_attachments: &Vec<TextureAttachmentData>,
        depth_texture: &Option<TextureAttachmentData>,
    ) -> Result<Self::PipelineInterface, MemoryError>;

    /// Creates a DescriptorPool with the desired descriptors
    ///
    /// # Parameters
    ///
    /// * `num_sampled_images` - The number of sampled image descriptors you'll make from the new pool
    /// * `num_samplers` - The number of sampler descriptors you'll make from the pool
    /// * `num_uniform_buffers` - The number of UBO/CBV or SSBO/UAV descriptors you'll make from the pool
    fn create_descriptor_pool(
        &self,
        num_sampled_images: u32,
        num_samplers: u32,
        num_uniform_buffers: u32,
    ) -> Result<Vec<Self::DescriptorPool>, DescriptorPoolCreationError>;

    /// Creates a Pipeline with the provided PipelineInterface and the given PipelineCreateInfo
    ///
    /// # Parameters
    ///
    /// * `pipeline_interface` - The interface you want the new pipeline to have
    /// * `data` - The data to create a pipeline from
    fn create_pipeline(
        &self,
        pipeline_interface: Self::PipelineInterface,
        data: PipelineData,
    ) -> Result<Self::Pipeline, PipelineCreationError>;

    /// Creates an Image from the specified ImageCreateInto
    ///
    /// Images are created directly from the Device and not from a MemoryPool because
    ///
    /// # Parameters
    ///
    /// * `data` - The ImageData to create the image from
    fn create_image(&self, data: shaderpack::TextureCreateInfo) -> Result<Self::Image, MemoryError>;
    fn create_image(&self, data: ImageData) -> Result<Self::Image, MemoryError>;

    /// Creates a new Semaphore
    fn create_semaphore(&self) -> Result<Self::Semaphore, MemoryError>;

    /// Creates the specified number of Semaphores
    ///
    /// # Parameters
    ///
    /// * `count` - The number of semaphores to create
    fn create_semaphores(&self, count: u32) -> Result<Vec<Self::Semaphore>, MemoryError>;

    /// Creates a new Fence
    fn create_fence(&self) -> Result<Self::Fence, MemoryError>;

    /// Creates the specified number of Fences
    ///
    /// # Parameters
    ///
    /// * `count` - The number of fences to create
    fn create_fences(&self, count: u32) -> Result<Vec<Self::Fence>, MemoryError>;

    /// Waits for all the provided fences to be signalled
    ///
    /// # Parameters
    ///
    /// * `fences` - All the fences to wait for
    fn wait_for_fences(&self, fences: Vec<Self::Fence>);

    /// Resets all the provided fences to an unsignalled state
    ///
    /// # Parameters
    ///
    /// * `fences` - The fences to reset
    fn reset_fences(&self, fences: Vec<Self::Fence>);

    /// Executes the provided DescriptorSetWrites on this device
    ///
    /// # Parameters
    ///
    /// * `updates` - The DescriptorSetWrites to execute
    fn update_descriptor_sets(&self, updates: Vec<DescriptorSetWrite>);
}

pub trait Queue {
    type CommandList: CommandList;
    type Fence: Fence;
    type Semaphore: Semaphore;

    /// Submits a command list to this queue
    ///
    /// # Parameters
    ///
    /// * `commands` - The CommandList to submit to this queue
    /// * `fence_to_signal` - The Fence to signal after the CommandList has finished executing
    /// * `wait_semaphores` The semaphores to wait for before executing the CommandList
    /// * `signal_semaphores` - The semaphores to signal when the CommandList has finished executing
    fn submit_commands(
        commands: Self::CommandList,
        fence_to_signal: Self::Fence,
        wait_semaphores: Vec<Self::Semaphore>,
        signal_semaphores: Vec<Self::Semaphore>,
    );
}

/// A block of memory and an allocation strategy
pub trait Memory {
    type Buffer: Buffer;

    /// Creates a buffer from this memory
    ///
    /// It's the caller's responsibility to make sure that this memory is allowed to create buffers
    ///
    /// # Parameters
    ///
    /// * `data` - The BufferData to create the new buffer from
    fn create_buffer(&self, data: BufferCreateInfo) -> Result<Self::Buffer, MemoryError>;
    fn create_buffer(&self, data: BufferData) -> Result<Self::Buffer, MemoryError>;
}

pub trait Resource {}

pub trait Buffer {
    /// Writes data to the specified region of this buffer
    ///
    /// Note: buffers you call this method on must _not_ be device local, because they must be
    /// CPU-addressable
    ///
    /// # Parameters
    ///
    /// * `data` - The data to write to the buffer
    /// * `num_bytes` - The number of bytes of the data to write
    /// * `offset` - The offset in the buffer to where you want the data to be
    fn write_data(&self, data: BufferData, num_bytes: u64, offset: u64);
}

pub trait Image {}

pub trait Sampler {}

/// A pool of descriptors
pub trait DescriptorPool {
    type PipelineInterface: PipelineInterface;
    type DescriptorSet: DescriptorSet;

    /// Creates DescriptorSets from the provided PipelineInterface
    ///
    /// # Parameters
    ///
    /// * `pipeline_interface` - The PipelineInterface to create the descriptors from
    fn create_descriptor_sets(&self, pipeline_interface: Self::PipelineInterface) -> Vec<Self::DescriptorSet>;
}

pub trait DescriptorSet {}

pub trait Renderpass {}

pub trait Framebuffer {}

pub trait PipelineInterface {}

pub trait Pipeline {}

pub trait Semaphore {}

pub trait Fence {}

pub trait CommandAllocator {
    type CommandList: CommandList;

    fn create_command_list() -> Result<Self::CommandList, MemoryError>;
}

/// A CommandList is a sequence of commands which can be submitted to the GPU
pub trait CommandList {
    type Buffer: Buffer;
    type CommandList: CommandList;
    type Renderpass: Renderpass;
    type Framebuffer: Framebuffer;
    type Pipeline: Pipeline;
    type DescriptorSet: DescriptorSet;
    type PipelineInterface: PipelineInterface;
    type Resource: Resource;

    /// Records resource barriers which happen after all the stages in the `stages_before_barrier`
    /// bitmask, and before all the stages in the `stages_after_barrier` bitmask
    ///
    ///# Parameters
    ///
    /// * `stages_before_barrier` - The pipeline barrier will take place after all the stages in this bitmask
    /// * `stages_after_barrier` - The pipeline barrier will take place before all the stages in this bitmask
    /// * `barriers` - The resource barriers to record
    fn resource_barriers(
        stages_before_barrier: PipelineStageFlags,
        stages_after_barrier: PipelineStageFlags,
        barriers: Vec<ResourceBarrier<Self::Resource>>,
    );

    /// Records a command to copy data from one buffer to another
    ///
    /// # Parameters
    ///
    /// * `destination_buffer` - The buffer to write data to
    /// * `destination_offset` - The number of bytes from the start of `destination_buffer` to write to
    /// * `source_buffer` - The buffer to read data from
    /// * `source_offset` - The number of bytes from the start of `source_buffer` to read data from
    /// * `num_bytes` - The number of bytes to copy
    fn copy_buffer(
        destination_buffer: Self::Buffer,
        destination_offset: u64,
        source_buffer: Self::Buffer,
        source_offset: u64,
        num_bytes: u64,
    );

    /// Records a command to execute the provided command lists
    ///
    /// # Parameters
    ///
    /// * `lists` - The command lists to execute
    fn execute_command_lists(lists: Vec<Self::CommandList>);

    /// Records a command to begin a renderpass with a framebuffer
    ///
    /// # Parameters
    ///
    /// * `renderpass` - The renderpass to begin
    /// * `framebuffer` - The framebuffer to begin the renderpass with
    fn begin_renderpass(renderpass: Self::Renderpass, framebuffer: Self::Framebuffer);

    /// Records a command to end the current renderpass
    fn end_renderpass();

    /// Binds a pipeline to the command list
    ///
    /// # Parameters
    ///
    /// * `pipeline` - The pipeline to bind
    fn bind_pipeline(pipeline: Self::Pipeline);

    /// Records a command to bind DescriptorSet to a PipelineInterface
    ///
    /// # Parameters
    ///
    /// * `descriptor_sets` - The DescriptorSets to bind
    /// * `pipeline_interface` - The PipelineInterface to bind the descriptor sets to
    fn bind_descriptor_sets(descriptor_sets: Vec<Self::DescriptorSet>, pipeline_interface: Self::PipelineInterface);

    /// Records a command to bind vertex buffers
    ///
    /// Vertex buffers are always bound sequentially starting at binding 0
    ///
    /// # Parameters
    ///
    /// * `buffers` - The buffers to bind
    fn bind_vertex_buffers(buffers: Vec<Self::Buffer>);

    /// Binds an index buffer
    ///
    /// # Parameters
    ///
    /// * `buffer` - The buffer to bind as an index buffer
    fn bind_index_buffer(buffer: Self::Buffer);

    /// Records a drawcall to grab `num_indices` indices from the currently bound index buffer and
    /// draw them `num_instances` times
    ///
    /// # Parameters
    ///
    /// * `num_indices` - The number of indices to draw from the currently bound index buffer
    /// * `num_instances` - How many times to draw the mesh
    fn draw_indexed_mesh(num_indices: u32, num_instances: u32);
}
