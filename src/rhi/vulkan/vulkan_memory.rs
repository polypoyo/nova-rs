use super::super::{BufferCreateInfo, Memory, MemoryError};
use ash::vk;

pub struct VulkanMemory {
    device: ash::Device,
    memory: vk::DeviceMemory,
}

impl Memory for VulkanMemory {
    type Buffer = ();

    fn create_buffer(&self, data: BufferCreateInfo) -> Result<Self::Buffer, MemoryError> {
        unimplemented!()
    }
}
