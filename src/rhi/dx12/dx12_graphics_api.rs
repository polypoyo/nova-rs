use crate::rhi::GraphicsApi;

use super::dx12_physical_device::Dx12PhysicalDevice;

pub struct Dx12GraphicsApi {}

impl GraphicsApi for Dx12GraphicsApi {
    type PhysicalDevice = Dx12PhysicalDevice;

    fn get_adapters() -> Vec<Dx12PhysicalDevice> {
        unimplemented!()
    }
}
