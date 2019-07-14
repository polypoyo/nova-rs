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

bitfield! {
    pub struct PipelineStageFlags(u32);
    impl Debug;

    u32;
    pub top_of_pipe, set_top_of_pipe: 0;
    pub draw_indirect, set_draw_indirect: 1;
    pub vertex_input, set_vertex_input: 2;
    pub vertex_shader, set_vertex_shader: 3;
    pub tessellation_control_shader, set_tessellation_control_shader: 4;
    pub tessellation_evaluation_shader, set_tessellation_evaluation_shader: 5;
    pub geometry_shader, set_geometry_shader: 6;
    pub fragment_shader, set_fragment_shader: 7;
    pub early_fragment_tests, set_early_fragment_tests: 8;
    pub late_fragment_tests, set_late_fragment_tests: 9;
    pub color_attachment_output, set_color_attachment_output: 10;
    pub compute_shader, set_compute_shader: 11;
    pub transfer, set_transfer: 12;
    pub bottom_of_pipe, set_bottom_of_pipe: 13;
    pub host, set_host: 14;
    pub all_graphics, set_all_graphics: 15;
    pub all_commands, set_all_commands: 16;
    pub shading_rate_image, set_shading_rate_image: 17;
    pub ray_tracing_shader, set_ray_tracing_shader: 18;
    pub acceleration_structure_build, set_acceleration_structure_build: 19;
    pub task_shader, set_task_shader: 20;
    pub mesh_shader, set_mesh_shader: 21;
    pub fragment_density_process, set_fragment_density_process: 22;
}
