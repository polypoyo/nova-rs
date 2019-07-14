use super::super::GraphicsApi;

use super::vulkan_physical_device::VulkanPhysicalDevice;

pub struct VulkanGraphicsApi {}

impl GraphicsApi for VulkanGraphicsApi {
    type PhysicalDevice = VulkanPhysicalDevice;

    fn get_adapters() -> Vec<VulkanPhysicalDevice> {
        unimplemented!()
    }
}
