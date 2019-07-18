use super::{super::*, vulkan_queue::VulkanQueue};
use crate::rhi::shaderpack::*;
use ash::{version::DeviceV1_0, vk};
use cgmath::Vector2;
use std::collections::{hash_map::RandomState, HashMap};

pub struct VulkanDevice {
    instance: ash::Instance,
    device: ash::Device,

    graphics_queue_family_index: u32,
    transfer_queue_family_index: u32,
    compute_queue_family_index: Option<u32>,
}

impl Device for VulkanDevice {
    type Queue = VulkanQueue;
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

    fn get_queue(&self, queue_type: QueueType, queue_index: u32) -> Result<Self::Queue, QueueGettingError> {
        let queue_family_index = match queue_type {
            QueueType::Graphics => self.graphics_queue_family_index,
            QueueType::Copy => self.transfer_queue_family_index,
            QueueType::Compute => {
                if self.compute_queue_family_index.is_some() {
                    self.compute_queue_family_index.unwrap()
                } else {
                    return Err(QueueGettingError::NotSupported);
                }
            }
        };

        assert_eq!(queue_index, 0, "Only queue index 0 is supported at the moment");
        let queue = unsafe { self.device.get_device_queue(queue_family_index, queue_index) };

        Ok(VulkanQueue { queue })
    }

    fn allocate_memory(
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

    fn create_renderpass(&self, data: RenderPassCreationInfo) -> Result<Self::Renderpass, MemoryError> {
        unimplemented!()
    }

    fn create_framebuffer(
        &self,
        renderpass: Self::Renderpass,
        attachments: Vec<Self::Image>,
        framebuffer_size: Vector2<f32>,
    ) -> Result<Self::Framebuffer, MemoryError> {
        unimplemented!()
    }

    fn create_pipeline_interface(
        &self,
        bindings: &HashMap<String, ResourceBindingDescription, RandomState>,
        color_attachments: &Vec<TextureAttachmentInfo>,
        depth_texture: &Option<TextureAttachmentInfo>,
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
        data: PipelineCreationInfo,
    ) -> Result<Self::Pipeline, PipelineCreationError> {
        unimplemented!()
    }

    fn create_image(&self, data: TextureCreateInfo) -> Result<Self::Image, MemoryError> {
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
