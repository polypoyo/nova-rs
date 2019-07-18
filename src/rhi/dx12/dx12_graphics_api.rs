use winapi::shared::{dxgi, dxgi1_4};

use crate::rhi::GraphicsApi;

use super::dx12_physical_device::Dx12PhysicalDevice;

#[derive(Debug, Clone, Eq)]
pub struct Dx12GraphicsApi {
    factory: native::WeakPtr<dxgi1_4::IDXGIFactory4>,
}

impl Dx12GraphicsApi {
    fn new() -> Self {
        let factory: dxgi1_4::IDXGIFactory4;
        unsafe {
            dxgi1_2::CreateDXGIFactory2(0, &dxgi1_4::IDXGIFactory4::uuidof(), factory.mut_void());
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
