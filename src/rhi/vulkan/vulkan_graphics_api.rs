use super::super::{GraphicsAPI, PhysicalDevice};

pub struct VulkanGraphicsAPI {}

impl GraphicsAPI for VulkanGraphicsAPI {
    fn get_adapters() -> Vec<PhysicalDevice> {
        unimplemented!()
    }
}
