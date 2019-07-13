//! Nova's Render Hardware Interface
//! 
//! This is an interface to the GPU which has been designed for Nova. It abstracts away parts of the underlying APIs 
//! which Nova doesn't use, providing an interface that's more productive and more fun. The RHI is actually split into
//! two sections: the synchronous parts and the asynchronous part. The synchronous part of the API is where your calls
//! happen immediately on the GPU, while the asynchronous part is where your calls get recorded into command lists, 
//! which are later executed on the GPU

use std::collections::HashMap;

/// An implementation of the rendering API. This will probably be a GPU card, but a software implementation of either 
/// Vulkan or Direct3D 12 is possible
pub trait PhysicalDevice {
    /// Initializes a PhysicalDevice, creating it from the graphics API
    /// 
    /// # Parameters
    /// 
    /// * `create_info` - Information about how you want the PhysicalDevice to be created
    fn new(create_info: PhysicalDeviceCreateInfo) -> Self;

    /// Retrieves information about the PhysicalDevice
    fn get_info(&self) -> PhysicalDeviceInfo;

    /// Creates a new logical Device
    /// 
    /// # Parameters
    /// 
    /// * `create_info` - Information about how you want the Device to be created
    // TODO: Maybe consume self?
    fn create_logical_device(&self, create_info: LogicalDeviceCreateInfo) -> Result<Device, DeviceCreationError>;
}

/// The logical device that we're rendering with
/// 
/// There may be multiple Devices in existence at once. Nova will eventually support multi-GPU rendering
pub trait Device {
    /// Retrieves the Queue with the provided queue family index and queue index
    /// 
    /// The caller should verify that the device supports the requested queue index and queue family index
    /// 
    /// # Parameters
    /// 
    /// * `queue_family_index` - The queue family index to get a queue from
    /// * `queue_index` - The index of the queue to get from the selected queue family
    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Result<Queue, QueueGettingError>;

    /// Allocates memory from the graphics API
    /// 
    /// This memory may be on the device or on the host, depending on its usage and allowed objects
    /// 
    /// # Parameters
    /// 
    /// * `size` - The size, in bytes, of the memory you want to allocate
    /// * `memory_usage` - The usage you want the memory to be usable for
    /// * `allowed_objects` - The types of objects you want to allow from this memory. Enforcing this is up to the caller
    fn allocate_memory(&self, size: u64, memory_usage: MemoryUsage, allowed_objects: ObjectType) -> Result<Memory, AllocationError>;

    /// Creates a new CommandPool
    /// 
    /// # Parameters
    /// 
    /// * `create_info` - Information about how you want the CommandPool created
    fn create_command_pool(&self, create_info: CommandPoolCreateInfo) -> Result<CommandPool, CommandPoolCreationError>;

    /// Creates a new renderpass from the provided shaderpack data
    /// 
    /// # Parameters
    /// 
    /// * `data` - The shaderpack data to create the renderpass from
    fn create_renderpass(&self, data: RenderpassData) -> Option<RenderPass>;

    /// Creates a new Framebuffer
    /// 
    /// Framebuffers get their attachment layout from a renderpass. I do not know why Khronos didn't make a separate 
    /// type for a framebuffer interace, yet here we are. Thus, this method takes in the renderpass to use an interface
    /// 
    /// # Parameters
    /// 
    /// * `renderpass` - The RenderPass to get the framebuffer layout from
    /// * `attachments` - The images to attach to the framebuffer, in attachment order
    /// * `framebuffer_size` - The size of the framebuffer, in pixels
    fn create_framebuffer(&self, renderpass: RenderPass, attachments: Vec<Image>, framebuffer_size: Vec2) -> Result<Framebuffer, FramebufferCreateError>;

    /// Creates a PipelineInterface from the provided information
    /// 
    /// # Parameters
    /// 
    /// * `bindings` - The bindings that the pipeline exposes
    /// * `color_attachemts` - All the color attachments that the pipline writes to
    /// * `depth_texture` - The depth texture that this pipeline writes to, if it writes to one
    fn create_pipeline_interface(&self, bindings: &HashMap<String, ResourceBindingDescription>, color_attachments: &Vec<TextureAttachmentInfo>,
                                 depth_texture: &Option<TextureAttachmentInfo>) -> Result<PipelineInterface, PipelineInterfaceCreationError>;

    /// Creates a DescriptorPool with the desired descriptors
    /// 
    /// # Parameters
    /// 
    /// * `num_sampled_images` - The number of sampled image descriptors you'll make from the new pool
    /// * `num_samplers` - The number of sampler descriptors you'll make from the pool
    /// * `num_uniform_buffers` - The number of UBO/CBV or SSBO/UAV descriptors you'll make from the pool
    fn create_descriptor_pool(&self, num_sampled_images: u32, num_samplers: u32, num_uniform_buffers: u32)
        -> Result<Vec<DescriptorPool>, DescriptorPoolCreateError>;

    /// Creates a Pipeline with the provided PipelineInterface and the given PipelineCreateInfo
    /// 
    /// # Parameters
    /// 
    /// * `pipeline_interface` - The interface you want the new pipeline to have
    /// * `create_info` - The information to create a pipeline from
    fn create_pipeline(&self, pipeline_interface: PipelineInterface, create_info: PipelineCreateInfo) -> Result<Pipeline, PipelineCreationError>;

    /// Creates an Image from the specified ImageCreateInto
    /// 
    /// # Parameters
    /// 
    /// * `create_info` - The ImageCrreateInfo to create the image from
    fn create_image(&self, create_info: ImageCreateInfo) -> Result<Image, ImageCreateError>;

    /// Creates a new Semaphore
    fn create_semaphore(&self) -> Result<Semaphore, SemaphoreCreateError>;

    /// Creates the specified number of Semaphores
    /// 
    /// # Parameters
    /// 
    /// * `count` - The number of semaphores to create
    fn create_semaphores(&self, count: u32) -> Result<Vec<Semaphore>, SemaphoreCreateError>;

    /// Creates a new Fence
    fn create_fence(&self) -> Result<Fence, FenceCreateError>;

    /// Creates the specified number of Fences
    /// 
    /// # Parameters
    /// 
    /// * `count` - The number of fences to create
    fn create_fences(&self, count: u32) -> Result<Vec<Fence>, FenceCreateError>;

    /// Executes the provided DescriptorSetWrites on this device
    /// 
    /// # Parameters
    /// 
    /// - `updates` - The DescriptorSetWrites to execute
    fn update_descriptor_sets(&self, updates: Vec<DescriptorSetWrite>);
}

pub trait Memory {
    /// Creates a buffer from this memory
    /// 
    /// It's the caller's responsibility to make sure that this memory is allowed to create buffers
    /// 
    /// # Parameters 
    /// 
    /// * `create_info` - The BufferCreateInfo to create the new buffer from
    fn create_buffer(&self, create_info: BufferCreateInfo) -> Result<Buffer, BufferCreateError>;
}

pub trait CommandPool {
}

/// A pool of descriptors
pub trait DescriptorPool {
    /// Creates DescriptorSets from the provided PipelineInterface
    /// 
    /// # Parameters
    /// 
    /// * `pipeline_interface` - The PipelineInterface to create the descriptors from
    fn create_descriptor_sets(&self, pipeline_interface: PipelineInterface) -> Vec<DescriptorSet>;
}

pub trait Buffer {
    /// Writes data to the specified region of this buffer
    /// 
    /// Note: buffers you call this method on must _not_ be device local, because they must be CPU-addressable
    /// 
    /// # Parameters
    /// 
    /// * `data` - The data to write to the buffer
    /// * `num_bytes` - The number of bytes of the data to write
    /// * `offset` - The offset in the buffer to where you want the data to be
    fn write_data(&self, data: BufferData, num_bytes: u64, offset: u64);
}