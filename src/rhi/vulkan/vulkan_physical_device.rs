use super::super::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

pub struct VulkanPhysicalDevice {}

impl PhysicalDevice for VulkanPhysicalDevice {
    type Device = ();

    fn get_properties(&self) -> PhysicalDeviceProperties {
        unimplemented!()
    }

    fn can_be_used_by_nova(&self) -> bool {
        unimplemented!()
    }

    fn create_logical_device(&self) -> Result<Self::Device, DeviceCreationError> {
        unimplemented!()
    }
}
