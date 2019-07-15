use crate::rhi::{Memory, MemoryError};

pub struct Dx12Memory {}

impl Memory for Dx12Memory {
    type Buffer = ();

    fn create_buffer(&self, data: _) -> Result<Self::Buffer, MemoryError> {
        unimplemented!()
    }
}
