use crate::{
    rhi::{
        dx12::{
            dx12_command_allocator::Dx12CommandAllocator, dx12_descriptor_pool::Dx12DescriptorPool,
            dx12_fence::Dx12Fence, dx12_framebuffer::Dx12Framebuffer, dx12_image::Dx12Image, dx12_memory::Dx12Memory,
            dx12_physical_device::Dx12PhysicalDevice, dx12_pipeline::Dx12Pipeline,
            dx12_pipeline_interface::Dx12PipelineInterface, dx12_queue::Dx12Queue, dx12_renderpass::Dx12Renderpass,
            dx12_semaphore::Dx12Semaphore,
        },
        AllocationError, CommandAllocatorCreateInfo, DescriptorPoolCreationError, DescriptorSetWrite, Device,
        MemoryError, MemoryUsage, ObjectType, PhysicalDevice, PipelineCreationError, QueueGettingError,
        ResourceBindingDescription,
    },
    shaderpack,
};
use cgmath::Vector2;
use std::collections::{hash_map::RandomState, HashMap};

pub struct Dx12Device {}

impl Device for Dx12Device {
    type Queue = Dx12Queue;
    type Memory = Dx12Memory;
    type CommandAllocator = Dx12CommandAllocator;
    type Image = Dx12Image;
    type Renderpass = Dx12Renderpass;
    type Framebuffer = Dx12Framebuffer;
    type PipelineInterface = Dx12PipelineInterface;
    type DescriptorPool = Dx12DescriptorPool;
    type Pipeline = Dx12Pipeline;
    type Semaphore = Dx12Semaphore;
    type Fence = Dx12Fence;

    fn get_queue(&self, queue_family_index: u32, queue_index: u32) -> Result<Dx12Queue, QueueGettingError> {
        unimplemented!()
    }

    fn allocate_memory<T>(
        &self,
        size: u64,
        memory_usage: MemoryUsage,
        allowed_objects: ObjectType,
    ) -> Result<Dx12Memory, AllocationError> {
        unimplemented!()
    }

    fn create_command_allocator(
        &self,
        create_info: CommandAllocatorCreateInfo,
    ) -> Result<Dx12CommandAllocator, MemoryError> {
        unimplemented!()
    }

    fn create_renderpass(&self, data: shaderpack::RenderPassCreationInfo) -> Result<Dx12Renderpass, MemoryError> {
        unimplemented!()
    }

    fn create_framebuffer(
        &self,
        renderpass: Dx12Renderpass,
        attachments: Vec<Dx12Image>,
        framebuffer_size: Vector2<f32>,
    ) -> Result<Dx12Framebuffer, MemoryError> {
        unimplemented!()
    }

    fn create_pipeline_interface(
        &self,
        bindings: &HashMap<String, ResourceBindingDescription>,
        color_attachments: &Vec<shaderpack::TextureAttachmentInfo>,
        depth_texture: &Option<shaderpack::TextureAttachmentInfo>,
    ) -> Result<Dx12PipelineInterface, MemoryError> {
        unimplemented!()
    }

    fn create_descriptor_pool(
        &self,
        num_sampled_images: u32,
        num_samplers: u32,
        num_uniform_buffers: u32,
    ) -> Result<Vec<Dx12DescriptorPool>, DescriptorPoolCreationError> {
        unimplemented!()
    }

    fn create_pipeline(
        &self,
        pipeline_interface: Dx12PipelineInterface,
        data: shaderpack::PipelineCreationInfo,
    ) -> Result<Dx12Pipeline, PipelineCreationError> {
        unimplemented!()
    }

    fn create_image(&self, data: shaderpack::TextureCreateInfo) -> Result<Dx12Image, MemoryError> {
        unimplemented!()
    }

    fn create_semaphore(&self) -> Result<Dx12Semaphore, MemoryError> {
        unimplemented!()
    }

    fn create_semaphores(&self, count: u32) -> Result<Vec<Dx12Semaphore>, MemoryError> {
        unimplemented!()
    }

    fn create_fence(&self) -> Result<Dx12Fence, MemoryError> {
        unimplemented!()
    }

    fn create_fences(&self, count: u32) -> Result<Vec<Dx12Fence>, MemoryError> {
        unimplemented!()
    }

    fn wait_for_fences(&self, fences: Vec<Dx12Fence>) {
        unimplemented!()
    }

    fn reset_fences(&self, fences: Vec<Dx12Fence>) {
        unimplemented!()
    }

    fn update_descriptor_sets(&self, updates: Vec<DescriptorSetWrite>) {
        unimplemented!()
    }
}
