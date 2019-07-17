use super::super::*;
use std::collections::HashMap;

pub struct VulkanDevice;

impl Device for VulkanDevice {
    type Queue = ();
    type Memory = ();
    type CommandAllocator = ();
    type Image = ();
    type Renderpass = ();
    type Framebuffer = ();
    type PipelineInterface = ();
    type DescriptorPool = ();
    type Pipeline = ();
    type Semaphore = ();
    type Fence = ();

    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Result<Self::Queue, QueueGettingError> {
        unimplemented!()
    }

    fn allocate_memory<T>(
        &self,
        size: u64,
        memory_usage: MemoryUsage,
        allowed_objects: ObjectType,
    ) -> Result<Self::Memory, AllocationError> {
        unimplemented!()
    }

    fn create_command_allocator(
        &self,
        create_info: CommandAllocatorCreateInfo,
    ) -> Result<Self::CommandAllocator, MemoryError> {
        unimplemented!()
    }

    fn create_renderpass(&self, data: RenderpassData) -> Result<Self::Renderpass, MemoryError> {
        unimplemented!()
    }

    fn create_framebuffer(
        &self,
        renderpass: Self::Renderpass,
        attachments: Vec<Self::Image>,
        framebuffer_size: _,
    ) -> Result<Self::Framebuffer, MemoryError> {
        unimplemented!()
    }

    fn create_pipeline_interface(
        &self,
        bindings: &HashMap<String, ResourceBindingDescription>,
        color_attachments: &Vec<TextureAttachmentData>,
        depth_texture: &Option<TextureAttachmentData>,
    ) -> Result<Self::PipelineInterface, MemoryError> {
        unimplemented!()
    }

    fn create_descriptor_pool(
        &self,
        num_sampled_images: u32,
        num_samplers: u32,
        num_uniform_buffers: u32,
    ) -> Result<Vec<Self::DescriptorPool>, DescriptorPoolCreationError> {
        unimplemented!()
    }

    fn create_pipeline(
        &self,
        pipeline_interface: Self::PipelineInterface,
        data: _,
    ) -> Result<Self::Pipeline, PipelineCreationError> {
        unimplemented!()
    }

    fn create_image(&self, data: ImageData) -> Result<Self::Image, MemoryError> {
        unimplemented!()
    }

    fn create_semaphore(&self) -> Result<Self::Semaphore, MemoryError> {
        unimplemented!()
    }

    fn create_semaphores(&self, count: u32) -> Result<Vec<Self::Semaphore>, MemoryError> {
        unimplemented!()
    }

    fn create_fence(&self) -> Result<Self::Fence, MemoryError> {
        unimplemented!()
    }

    fn create_fences(&self, count: u32) -> Result<Vec<Self::Fence>, MemoryError> {
        unimplemented!()
    }

    fn wait_for_fences(&self, fences: Vec<Self::Fence>) {
        unimplemented!()
    }

    fn reset_fences(&self, fences: Vec<Self::Fence>) {
        unimplemented!()
    }

    fn update_descriptor_sets(&self, updates: Vec<DescriptorSetWrite>) {
        unimplemented!()
    }
}
