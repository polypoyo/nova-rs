use super::rhi_enums::CommandListType;

/// Describes how to create a logical device
pub struct LogicalDeviceCreateInfo {

}

/// Describes what kind of command allocator you want to create
pub struct CommandAllocatorCreateInfo {
    /// The type of command lists which will be allocated by this command allocator
    command_list_type: CommandListType,

    // A bitmask of the GPU that the new command allocator will allocate commands for. Only one GPU mey be used
    node_mask: u32,
}

/// Information about a physical device!
/// 
/// This information can come from multiple API calls, but I've merged all the information together here
/// 
/// This structure has things like the capabilities of the device, its hardware limits, its manufacturer and model
/// number, etc
pub struct PhysicalDeviceProperties {
    manufacturer: PhysicalDeviceManufacturer,

    device_id: u32,

    device_name: str,

    device_type: PhysicalDeviceType,

    max_color_attachments: u32,
}

