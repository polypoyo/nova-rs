use crate::rhi::{
    dx12::{dx12_command_list::Dx12CommandList, dx12_fence::Dx12Fence, dx12_semaphore::Dx12Semaphore},
    Queue,
};

pub struct Dx12Queue {}

impl Queue for Dx12Queue {
    type CommandList = Dx12CommandList;
    type Fence = Dx12Fence;
    type Semaphore = Dx12Semaphore;

    fn submit_commands(
        commands: Dx12CommandList,
        fence_to_signal: Dx12Fence,
        wait_semaphores: Vec<Dx12Semaphore>,
        signal_semaphores: Vec<Dx12Semaphore>,
    ) {
        unimplemented!()
    }
}
