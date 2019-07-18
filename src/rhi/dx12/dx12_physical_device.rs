use crate::rhi::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use super::dx12_device::Dx12Device;
use d3d12::Device;
use winapi::shared::{dxgi1_2, winerror};

/// A physical device which supports DX12
pub struct Dx12PhysicalDevice {
    adapter: d3d12::WeakPtr<dxgi1_2::IDXGIAdapter2>,
}

impl Dx12PhysicalDevice {
    pub fn new(adapter: d3d12::WeakPtr<dxgi1_2::IDXGIAdapter2>) -> Self {
        Dx12PhysicalDevice { adapter }
    }
}

impl PhysicalDevice for Dx12PhysicalDevice {
    type Device = Dx12Device;

    fn get_properties(&self) -> PhysicalDeviceProperties {
        unimplemented!()
    }

    fn can_be_used_by_nova(&self) -> bool {
        // TODO: Something more in depth
        match self.create_logical_device() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn create_logical_device(&self) -> Result<Dx12Device, DeviceCreationError> {
        let (device, hr) = d3d12::Device::create(self.adapter, d3d12::FeatureLevel::L11_0);
        if winerror::SUCCEEDED(hr) {
            Ok(Dx12Device::new(device))
        } else {
            Err(DeviceCreationError::Failed)
        }
    }
}
