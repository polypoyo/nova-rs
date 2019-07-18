use super::super::Queue;

pub struct VulkanQueue;

impl Queue for VulkanQueue {
    type CommandList = ();
    type Fence = ();
    type Semaphore = ();

    fn submit_commands(
        commands: Self::CommandList,
        fence_to_signal: Self::Fence,
        wait_semaphores: Vec<Self::Semaphore>,
        signal_semaphores: Vec<Self::Semaphore>,
    ) {
        unimplemented!()
    }
}
