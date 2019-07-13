use super::rhi_enums::CommandListType;

/// Describes how to create a physical device
pub struct PhysicalDeviceCreateInfo {

}

/// Describes what kind of command allocator you want to create
pub struct CommandAllocatorCreateInfo {
    /// The type of command lists which will be allocated by this command allocator
    commandListType: CommandListType,

    // A bitmask of the GPU that the new command allocator will allocate commands for. Only one GPU mey be used
    nodeMask: u32,
}