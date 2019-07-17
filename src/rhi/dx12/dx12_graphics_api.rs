use dxgi;

use crate::rhi::GraphicsApi;

use super::dx12_physical_device::Dx12PhysicalDevice;

#[derive(Debug, Clone, Eq)]
pub struct Dx12GraphicsApi {}

impl Dx12GraphicsApi {
    fn new() -> Self {

        let dxgi_adapter: dxgi::IDXGIAdapter2;
        unsafe {
            dxgi::CreateDXGIFactory2(0, #[iid_ppv_args(&dxgi_adapter)]);
        }

        Dx12GraphicsApi{}
    }
}

impl GraphicsApi for Dx12GraphicsApi {
    type PhysicalDevice = Dx12PhysicalDevice;

    fn get_adapters() -> Vec<Dx12PhysicalDevice> {
        let mut adapters: Vec<Dx12PhysicalDevice>;
    }
}
