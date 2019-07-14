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

/// The type of command list
pub enum CommandListType {
    /// Graphics command lists can
    Primary,
    Secondary,
    Compute,
    Copy,
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
