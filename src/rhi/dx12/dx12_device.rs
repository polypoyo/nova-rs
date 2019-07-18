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
        MemoryError, MemoryUsage, ObjectType, PhysicalDevice, PipelineCreationError, QueueGettingError, QueueType,
        ResourceBindingDescription,
    },
    shaderpack,
};
use cgmath::Vector2;
use std::{
    collections::{hash_map::RandomState, HashMap},
    mem,
};
use winapi::{
    shared::{dxgi1_2, dxgi1_4},
    um::d3d12 as d3d12_raw,
};

pub struct Dx12Device {
    /// Graphics adapter that we're using
    phys_device: &Dx12PhysicalDevice,

    /// D3D12 device that we're wrapping
    device: d3d12::Device,

    /// Increment size of an RTV descriptor
    rtv_descriptor_size: u32,

    /// Increment size of a CBV, UAV, or SRV descriptor
    shader_resource_descriptor_size: u32,
}

impl Dx12Device {
    pub fn new(phys_device: &Dx12PhysicalDevice, device: d3d12::Device) -> Self {
        let rtv_descriptor_size = device.get_descriptor_handle_increment_size(d3d12::descriptor::HeapType::Rtv);
        let shader_resource_descriptor_size =
            device.get_descriptor_handle_increment_size(d3d12::descriptor::HeapType::CbvSrvUav);

        Dx12Device {
            phys_device,
            device,
            rtv_descriptor_size,
            shader_resource_descriptor_size,
        }
    }
}

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

    fn get_queue(&self, queue_type: QueueType, queue_index: u32) -> Result<Dx12Queue, QueueGettingError> {
        let queue_type = match queue_type {
            QueueType::Graphics => d3d12::command_list::CmdListType::Direct,
            QueueType::Compute => d3d12::command_list::CmdListType::Compute,
            QueueType::Copy => d3d12::command_list::CmdListType::Copy,
        };

        let (queue, hr) = self.device.create_command_queue(
            queue_type,
            d3d12::queue::Priority::Normal,
            d3d12::queue::CommandQueueFlags::empty(),
            0,
        );
        if winerror::SUCCESS(hr) {
            Ok(Dx12Queue::new(queue))
        } else {
            Err(QueueGettingError::OutOfMemory)
        }
    }

    fn allocate_memory(
        &self,
        size: u64,
        memory_usage: MemoryUsage,
        allowed_objects: ObjectType,
    ) -> Result<Dx12Memory, AllocationError> {
        let heap_properties = match memory_usage {
            MemoryUsage::DeviceOnly => d3d12_raw::D3D12_HEAP_PROPERTIES {
                Type: d3d12_raw::D3D12_HEAP_TYPE_DEFAULT,
                CPUPageProperty: d3d12_raw::D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
                MemoryPoolPreference: d3d12_raw::D3D12_MEMORY_POOL_L1, // TODO: Find a way to handle UMA
                CreationNodeMask: 0,
                VisibleNodeMask: 0,
            },
            MemoryUsage::LowFrequencyUpload => d3d12_raw::D3D12_HEAP_PROPERTIES {
                Type: d3d12_raw::D3D12_HEAP_TYPE_UPLOAD,
                CPUPageProperty: d3d12_raw::D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
                MemoryPoolPreference: d3d12_raw::D3D12_MEMORY_POOL_L1, // TODO: Find a way to handle UMA
                CreationNodeMask: 0,
                VisibleNodeMask: 0,
            },
            MemoryUsage::StagingBuffer => d3d12_raw::D3D12_HEAP_PROPERTIES {
                Type: d3d12_raw::D3D12_HEAP_TYPE_UPLOAD,
                CPUPageProperty: d3d12_raw::D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
                MemoryPoolPreference: d3d12_raw::D3D12_MEMORY_POOL_L0,
                CreationNodeMask: 0,
                VisibleNodeMask: 0,
            },
        };

        let heap_flags = match allowed_objects {
            ObjectType::Buffer => d3d12_raw::D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS,
            ObjectType::Texture => d3d12_raw::D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES,
            ObjectType::Attachment => d3d12_raw::D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES,
            ObjectType::SwapchainSurface => {
                d3d12_raw::D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES | d3d12_raw::D3D12_HEAP_FLAG_ALLOW_DISPLAY
            }
            ObjectType::Any => d3d12_raw::D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES,
        };

        // Ensure we have enough free memory for the requested allocation
        let free_memory = self.phys_device.get_free_memory();
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
