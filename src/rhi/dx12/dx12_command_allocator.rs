use crate::rhi::{dx12::dx12_command_list::Dx12CommandList, CommandAllocator, MemoryError};

pub struct Dx12CommandAllocator {}

impl CommandAllocator for Dx12CommandAllocator {
    type CommandList = Dx12CommandList;

    fn create_command_list() -> Result<Dx12CommandList, MemoryError> {
        unimplemented!()
    }
}
