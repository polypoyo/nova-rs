use crate::rhi::{dx12::dx12_buffer::Dx12Buffer, BufferCreateInfo, Memory, MemoryError};

pub struct Dx12Memory {}

impl Memory for Dx12Memory {
    type Buffer = Dx12Buffer;

    fn create_buffer(&self, data: BufferCreateInfo) -> Result<Self::Buffer, MemoryError> {
        unimplemented!()
    }
}
