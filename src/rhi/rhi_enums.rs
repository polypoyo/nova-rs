#[macro_use]
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

<<<<<<< HEAD
pub enum QueueType {
=======
/// The type of command list
pub enum CommandListType {
    /// Graphics command lists can
>>>>>>> [rhi] Use pointers instead of dyn, and some more trait work
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

bitflags! {
    pub struct PipelineStageFlags: u32 {
        const TopOfPipe = 0x00000001;
        const DrawIndirect = 0x00000002;
        const VertexInput = 0x00000004;
        const VertexShader = 0x00000008;
        const TessellationControlShader = 0x00000010;
        const TessellationEvaluationShader = 0x00000020;
        const GeometryShader = 0x00000040;
        const FragmentShader = 0x00000080;
        const EarlyFragmentTests = 0x00000100;
        const LateFragmentTests = 0x00000200;
        const ColorAttachmentOutput = 0x00000400;
        const ComputeShader = 0x00000800;
        const Transfer = 0x00001000;
        const BottomOfPipe = 0x00002000;
        const Host = 0x00004000;
        const AllGraphics = 0x00008000;
        const AllCommands = 0x00010000;
        const ShadingRateImage = 0x00400000;
        const RayTracingShader = 0x00200000;
        const AccelerationStructureBuild = 0x02000000;
        const TaskShader = 0x00080000;
        const MeshShader = 0x00100000;
        const FragmentDensityProcess = 0x00800000;
    }
}

bitflags! {
    pub struct ResourceAccessFlags: u32 {
        const NoFlags = 0x00000000;
        const IndexReadBit = 0x00000002;
        const VertexAttributeReadBit = 0x00000004;
        const UniformReadBit = 0x00000008;
        const InputAttachmentReadBit = 0x00000010;
        const ShaderReadBit = 0x00000020;
        const ShaderWriteBit = 0x00000040;
        const ColorAttachmentReadBit = 0x00000080;
        const ColorAttachmentWriteBit = 0x00000100;
        const DepthStencilAttachmentReadBit = 0x00000200;
        const DepthStencilAttachmentWriteBit = 0x00000400;
        const TransferReadBit = 0x00000800;
        const TransferWriteBit = 0x00001000;
        const HostReadBit = 0x00002000;
        const HostWriteBit = 0x00004000;
        const MemoryReadBit = 0x00008000;
        const MemoryWriteBit = 0x00010000;
    }
}

bitflags! {
    pub struct ImageAspectFlags: u32 {
         const Color = 0x00000001;
         const Depth = 0x00000002;
         const Stencil = 0x00000004;
    }
}
