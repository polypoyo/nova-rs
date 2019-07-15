use bitflags::bitflags;

pub enum PhysicalDeviceManufacturer {
    Nvidia,
    AMD,
    Intel,
    Other,
}

pub enum PhysicalDeviceType {
    Integrated,
    Discreet,
    Virtual,
    CPU,
    Other,
}

/// How a piece of memory will be used
pub enum MemoryUsage {
    /// The memory will only be used by device
    ///
    /// Useful for things like vertex buffers and dynamic textures
    DeviceOnly,

    /// The memory will be written to by the CPU, but will only be written to a handful of times per frame
    ///
    /// Useful for the model matrix buffer, the per-frame data buffer, and other uniform buffers which are updated a
    /// few times per frame
    LowFrequencyUpload,

    /// The memory will be used for a staging buffer
    StagingBuffer,
}

/// Describes what kind of object you want to allocate from a new memory pool
pub enum ObjectType {
    Buffer,
    Texture,
    Attachment,
    SwapchainSurface,
    Any,
}

pub enum QueueType {
    Graphics,
    Compute,
    Copy,
}

pub enum CommandListLevel {
    Primary,
    Secondary,
}

pub enum DeviceCreationError {
    Failed,
}

/// A memory-related error
pub enum MemoryError {
    /// There's not enough host memory to create the requested object
    OutOfHostMemory,

    /// There's not enough device memory to create the requested object
    OutOfDeviceMemory,
}

/// Errors tha can happen when you try to get a queue from a device
pub enum QueueGettingError {
    /// The device does not have enough memory to get you the queue you want
    OutOfMemory,
}

/// All the errors you might get when allocating memory
pub enum AllocationError {
    /// There's not enough host memory to make the requested allocation
    OutOfHostMemory,

    /// There's not enough device memory to make the requested allocation
    OutOfDeviceMemory,

    /// You've made too many memory allocations already
    TooManyObjects,

    InvalidExternalHandle,
}

pub enum DescriptorPoolCreationError {
    /// There's not enough host memory to create the descriptor pool
    OutOfHostMemory,

    /// There's not enough device memory to create the descriptor pool
    OutOfDeviceMemory,

    /// Memory is too fragmented to create the descriptor pool
    Fragmentation,
}

pub enum PipelineCreationError {
    /// There's not enough host memory to create the pipeline
    OutOfHostMemory,

    /// There's not enough device memory to create the pipeline
    OutOfDeviceMemory,

    /// One or more shaders failed to compile or link. If debug reports are enabled, details are
    /// reported through a debug report
    InvalidShader,
}

/// The state a resource is in
pub enum ResourceState {
    /// The state is not defined. The GPU may or may not do _things_ with the resource
    Undefined,
    /// The resource may be used for anything you want, but it won't be optimal for anything
    General,

    ColorAttachment,
    DepthStencilAttachment,
    DepthReadOnlyStencilAttachment,
    DepthAttachmentStencilReadOnly,
    DepthStencilReadOnlyAttachment,

    PresentSource,

    NonFragmentShaderReadOnly,
    FragmentShaderReadOnly,

    TransferSource,
    TransferDestination,
}

pub enum DescriptorType {
    CombinedImageSampler,
    UniformBuffer,
    StorageBuffer,
}

pub enum BufferUsage {
    UniformBuffer,
    IndexBuffer,
    VertexBuffer,
    StagingBuffer,
}

bitflags! {
    pub struct PipelineStageFlags: u32 {
        const TOP_OF_PIPE = 0x00000001;
        const DRAW_INDIRECT = 0x00000002;
        const VERTEX_INPUT = 0x00000004;
        const VERTEX_SHADER = 0x00000008;
        const TESSELLATION_CONTROL_SHADER = 0x00000010;
        const TESSELLATION_EVALUATION_SHADER = 0x00000020;
        const GEOMETRY_SHADER = 0x00000040;
        const FRAGMENT_SHADER = 0x00000080;
        const EARLY_FRAGMENT_TESTS = 0x00000100;
        const LATE_FRAGMENT_TESTS = 0x00000200;
        const COLOR_ATTACHMENT_OUTPUT = 0x00000400;
        const COMPUTE_SHADER = 0x00000800;
        const TRANSFER = 0x00001000;
        const BOTTOM_OF_PIPE = 0x00002000;
        const HOST = 0x00004000;
        const ALL_GRAPHICS = 0x00008000;
        const ALL_COMMANDS = 0x00010000;
        const SHADING_RATE_IMAGE = 0x00400000;
        const RAY_TRACING_SHADER = 0x00200000;
        const ACCELERATION_STRUCTURE_BUILD = 0x02000000;
        const TASK_SHADER = 0x00080000;
        const MESH_SHADER = 0x00100000;
        const FRAGMENT_DENSITY_PROCESS = 0x00800000;
    }
}

bitflags! {
    pub struct ResourceAccessFlags: u32 {
        const NO_FLAGS = 0x00000000;
        const INDEX_READ_BIT = 0x00000002;
        const VERTEX_ATTRIBUTE_READ_BIT = 0x00000004;
        const UNIFORM_READ_BIT = 0x00000008;
        const INPUT_ATTACHMENT_READ_BIT = 0x00000010;
        const SHADER_READ_BIT = 0x00000020;
        const SHADER_WRITE_BIT = 0x00000040;
        const COLOR_ATTACHMENT_READ_BIT = 0x00000080;
        const COLOR_ATTACHMENT_WRITE_BIT = 0x00000100;
        const DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200;
        const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400;
        const TRANSFER_READ_BIT = 0x00000800;
        const TRANSFER_WRITE_BIT = 0x00001000;
        const HOST_READ_BIT = 0x00002000;
        const HOST_WRITE_BIT = 0x00004000;
        const MEMORY_READ_BIT = 0x00008000;
        const MEMORY_WRITE_BIT = 0x00010000;
    }
}

bitflags! {
    pub struct ImageAspectFlags: u32 {
        const COLOR = 0x00000001;
        const DEPTH = 0x00000002;
        const STENCIL = 0x00000004;
    }
}
bitflags! {
    pub struct ShaderStageFlags: u32 {
        const VERTEX = 0x0001;
        const TESSELLATION_CONTROL = 0x0002;
        const TESSELLATION_EVALUATION = 0x0004;
        const GEOMETRY = 0x0008;
        const FRAGMENT = 0x0010;
        const COMPUTE = 0x0020;
        const RAYGEN = 0x0100;
        const ANY_HIT = 0x0200;
        const CLOSEST_HIT = 0x0400;
        const MISS = 0x0800;
        const INTERSECTION = 0x1000;
        const TASK = 0x0040;
        const MESH = 0x0080;
    }
}
