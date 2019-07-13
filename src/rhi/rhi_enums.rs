/// How a piece of memory will be used
pub enum MemoryUsage {
    /// The memory will only be used by device
    /// 
    /// Useful for things like vertex buffers and dynamic textures
    DeviceOnly,

    /// The memory will be written to by the CPU, but will onyl be written to a handful of times per frame
    /// 
    /// Useful for the model matrix buffer, the per-frame data buffer, and other uniform buffers which are updated a 
    /// few times per frame
    LowFrequencyUpload,

    /// The memory will be used for a stading buffer
    StagingBuffer
}

/// Describes what kind of object you want to allocate from a new memory pool
pub enum ObjectType {
    Buffer,
    Texture,
    Attachment,
    SwapchainSurface,
    Any
}

/// The type of command list
pub enum CommandListType {
    /// Graphics command lists can 
    Primary,
    Secondary,
    Compute,
    Copy
}