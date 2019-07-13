//! Nova's Render Hardware Interface
//! 
//! This is an interface to the GPU which has been designed for Nova. It abstracts away parts of the underlying APIs 
//! which Nova doesn't use, providing an interface that's more productive and more fun. The RHI is actually split into
//! two sections: the syncronous parts and the asynchronous part. The synchronous part of the API is where your calls 
//! happen immediately on the GPU, while the asynchronous part is where your calls get recorded into command lists, 
//! which are later executed on the GPU

use std::collections::HashMap;

/// An implementation of the rendering API. This will probably be a GPU card, but a software implementation of either 
/// Vulkan or Direct3D 12 is possible
trait PhysicalDevice {
    /// Initializes a PhysicalDevice, creating it from the graphics API
    /// 
    /// # Parameters
    /// 
    /// * `createInfo` - Information about how you want the PhysicalDevice to be created
    fn new(createInfo) -> Self;

    /// Retrieves informaition about the PhysicalDevice
    fn getInfo() -> PhysicalDeviceInfo*;

    /// Creates a new logical Device
    /// 
    /// # Parameters
    /// 
    /// * `createInfo` - Information about how you want the Device to be created
    fn createLogicalDevice(createInfo) -> Result<Device, DeviceCreationError>;
}

/// The logical device that we're rendering with
/// 
/// There may be multiple Devices in existance at once. Nova will eventually support multi-GPU rendering
trait Device {
    /// Retrieves the Queue with the provided queue family index and queue index
    /// 
    /// The caller should verify that the device supports the requested queue index and queue family index
    /// 
    /// # Parameters
    /// 
    /// * `queueFamilyIndex` - The queue family index to get a queue from
    /// * `queueIndex` - The index of the queue to get from the selected queue family
    fn getQueue(queueFamilyIndex: u32, queueIndex: u32) -> Result<Queue, QueueGettingError>;

    /// Allocates memory from the graphics API
    /// 
    /// This memory may be on the device or on the host, depending on its usage and allowed objects
    /// 
    /// # Parameters
    /// 
    /// * `size` - The size, in bytes, of the memory you want to allocate
    /// * `type` - The type of memory you want to allocate
    /// * `allowedObjects` - The types of objects you want to allow from this memory. Enforcing this is up to the caller
    fn allocateMemory(size: u64, type: MemoryUsage, allowedObjects: ObjectType) -> Result<Memory, AllocationError>;

    /// Creates a new CommandPpool
    /// 
    /// # Parameters
    /// 
    /// * `createInfo` - Information about how you want the CommandPool created
    fn createCommandPool(createInfo) -> Result<CommandPool, CommandPoolCreationError>;

    /// Creates a new renderpass from the provided shaderpack data
    /// 
    /// # Parameters
    /// 
    /// * `data` - The shaderpack data to create the renderpass from
    fn createRenderpass(data) -> Option<RenderPass>;

    /// Creates a new Framebuffer
    /// 
    /// Framebuffers get their attachment layout from a renderpass. I do not know why Khronos didn't make a separate 
    /// type for a framebuffer interace, yet here we are. Thus, this method takes in the renderpass to use an interface
    /// 
    /// # Parameters
    /// 
    /// * `renderpass` - The RenderPass to get the framebuffer layout from
    /// * `attachments` - The images to attach to the framebuffer, in attachment order
    /// * `framebufferSize` - The size of the framebuffer, in pixels
    fn createFramebuffer(renderpass: RenderPass, attachments: Vec<Image>, framebufferSize: Vec2) -> Result<Framebuffer, FramebufferCreateError>;

    /// Creates a PipelineInterface from the provided information
    /// 
    /// # Parameters
    /// 
    /// * `bindings` - The bindings that the pipeline exposes
    /// * `colorAttachemts` - All the color attachments that the pipline writes to
    /// * `depthTexture` - The depth texture that this pipeline writes to, if it writes to one
    fn createPipelineInterface(bindings: &HashMap<String, ResourceBindingDescription>, colorAttachments: &Vec<TextureAttachmentInfo>, depthTexture: &Option<TextureAttachmentInfo>) -> Result<PipelineInterface, PipelineInterfaceCreationError>;

    /// Creates a DescriptorPool with the desired descriptors
    /// 
    /// # Parameters
    /// 
    /// * `numSampledImages` - The number of sampled image descriptors you'll make from the new pool
    /// * `numSampler` - The number of sampler descriptors you'll make from the pool
    /// * `numUniformBuffers` - The number of UBO/CBV or SSBO/UAV descriptors you'll make from the pool
    fn createDescriptorPool(numSampledImages: u32, numSamplers: u32, numUniformBuffers: u32) -> Result<Vec<DescriptorPool>, DescriptorPoolCreateError>;

    /// Creates a Pipeline with the provided PipelineInterface and the given PipelineCreateInfo
    /// 
    /// # Parameters
    /// 
    /// * `pipelineInterface` - The interface you want the new pipeline to have
    /// * `createInfo` - The information to create a pipeline from
    fn createPipeline(pipelineInterface: PipelineInterface, createInfo: PipelineCreateInfo) -> Result<Pipeline, PipelineCreationError>;

    /// Creates an Image from the specified ImageCreateInto
    /// 
    /// # Parameters
    /// 
    /// * `createInfo` - The ImageCrreateInfo to create the image from
    fn createImage(createInfo: ImageCreateInfo) -> Result<Image, ImageCreateError>;

    /// Creates a new Semaphore
    fn createSemaphore() -> Result<Semaphore, SemaphoreCreateError>;

    /// Creates the specified number of Semaphores
    /// 
    /// # Parameters
    /// 
    /// * `count` - The number of semaphores to create
    fn createSemaphores(count: u32) -> Result<Vec<Semaphore>, SemaphoreCreateError>;

    /// Creates a new Fence
    fn createFence() -> Result<Fence, FenceCreateError>;

    /// Creates the specified number of Fences
    /// 
    /// # Parameters
    /// 
    /// * `count` - The number of fences to create
    fn createFences(count: u32) -> Result<Vec<Fence>, FenceCreateError>;

    /// Executes the provided DescriptorSetWrites on this device
    /// 
    /// # Parameters
    /// 
    /// - `updates` - The DescriptorSetWrites to execute
    fn updateDescriptorSets(updates: Vec<DescriptorSetWrite>);
}

trait Memory {
    /// Creates a buffer from this memory
    /// 
    /// It's the caller's responsibility to make sure that this memory is allowed to create buffers
    /// 
    /// # Parameters 
    /// 
    /// * `createInfo` - The BufferCreateInfo to create the new buffer from
    fn createBuffer(createInfo: BufferCreateInfo) -> Result<Buffer, BufferCreateError>;
}

trait CommandPool {
}

/// A pool of descriptors
trait DescriptorPool {
    /// Creates DescriptorSets from the provided PipelineInterface
    /// 
    /// # Parameters
    /// 
    /// * `pipelineInterface` - The PipelineInterface to create the descriptors from
    fn createDescriptorSets(pipelineInterface: PipelineInterface) -> Vec<DescriptorSet>;
}

trait Buffer {
    /// Writes data to the specified region of this buffer
    /// 
    /// Note: buffers you call this method on must _not_ be device local, because they must be CPU-addressable
    /// 
    /// # Parameters
    /// 
    /// * `data` - The data to write to the buffer
    /// * `numBytes` - The number of bytes of the data to write
    /// * `offset` - The offset in the buffer to where you want the data to be
    fn writeData(data, numBytes: u64, offset: u64);
}
