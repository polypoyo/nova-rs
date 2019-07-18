use d3d12;
use winapi::shared::{dxgi1_3, dxgi1_4, winerror};

use log::error;

use crate::rhi::GraphicsApi;

use super::dx12_physical_device::Dx12PhysicalDevice;

#[derive(Debug, Clone, Eq)]
pub struct Dx12GraphicsApi {
    factory: d3d12::WeakPtr<dxgi1_4::IDXGIFactory4>,
}

impl Dx12GraphicsApi {
    fn new() -> Self {
        let mut factory = d3d12::WeakPtr::<dxgi1_4::IDXGIFactory4>::null();
        let hr = unsafe { dxgi1_3::CreateDXGIFactory2(0, &dxgi1_4::IDXGIFactory4::uuidof(), factory.mut_void()) };

        if !winerror::SUCCEEDED(hr) {
            error!("Failed to create DXGI Factory: {:?}", hr);
        }

        Dx12GraphicsApi { factory }
    }
}

impl GraphicsApi for Dx12GraphicsApi {
    type PhysicalDevice = Dx12PhysicalDevice;

    fn get_adapters() -> Vec<Dx12PhysicalDevice> {
        let mut adapters: Vec<Dx12PhysicalDevice>;
    }
}
