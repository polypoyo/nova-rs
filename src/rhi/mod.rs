//! Nova's Render Hardware Interface
//!
//! This is an abstraction over Nova's supported APIs which presents an interface that was explicitly tailored to Nova.
//! This interface also hides some of the less tasteful parts of the supported APIs, such as the explicit memory
//! management. The RHI will be implemented by at least Vulkan and Direct3D 12. I'd like to eventually also support
//! Metal, but there's a lot to do before then

use super::shaderpack;

mod rhi_enums;
mod rhi_structs;
mod rhi_traits;

mod vulkan {
    // Only export the implementation of the GraphicsApi trait. Clients of Nova's RHI should only
    // use the API-specific structs to create a GraphicsApi, and for no other reason
    pub mod vulkan_graphics_api;

    // But we have to bring this into the mod.rs file so other code can use it

    mod vulkan_physical_device;
}

mod dx12 {
    pub mod dx12_graphics_api;

    mod dx12_buffer;
    mod dx12_command_allocator;
    mod dx12_command_list;
    mod dx12_descriptor_pool;
    mod dx12_descriptor_set;
    mod dx12_device;
    mod dx12_fence;
    mod dx12_framebuffer;
    mod dx12_image;
    mod dx12_memory;
    mod dx12_physical_device;
    mod dx12_pipeline;
    mod dx12_pipeline_interface;
    mod dx12_queue;
    mod dx12_renderpass;
    mod dx12_semaphore;
}

// Re-exports
pub use rhi_enums::*;
pub use rhi_structs::*;
pub use rhi_traits::*;

// Re-export entry points each supported API
pub use dx12::dx12_graphics_api::Dx12GraphicsApi;
pub use vulkan::vulkan_graphics_api::VulkanGraphicsApi;
