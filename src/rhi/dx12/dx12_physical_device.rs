use crate::rhi::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use super::dx12_device::Dx12Device;

pub struct Dx12PhysicalDevice {}

impl PhysicalDevice for Dx12PhysicalDevice {
    type Device = Dx12Device;

    fn get_properties(&self) -> PhysicalDeviceProperties {
        unimplemented!()
    }

    fn can_be_used_by_nova(&self) -> bool {
        unimplemented!()
    }

    fn create_logical_device(&self) -> Result<Dx12Device, DeviceCreationError> {
        unimplemented!()
    }
}
