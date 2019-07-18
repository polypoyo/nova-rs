use crate::rhi::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use super::dx12_device::Dx12Device;
use winapi::shared::dxgi1_2;

/// A physical device which supports DX12
pub struct Dx12PhysicalDevice {
    adapter: dxgi1_2::IDXGIAdapter2,
}

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
