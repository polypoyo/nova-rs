use crate::rhi::{Buffer, BufferCreateInfo};

pub struct Dx12Buffer {}

impl Buffer for Dx12Buffer {
    fn write_data(&self, data: BufferCreateInfo, num_bytes: u64, offset: u64) {
        unimplemented!()
    }
}
