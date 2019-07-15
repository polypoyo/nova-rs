//! Nova's Render Hardware Interface
//!
//! This is an abstraction over Nova's supported APIs which presents an interface that was explicitly tailored to Nova.
//! This interface also hides some of the less tasteful parts of the supported APIs, such as the explicit memory
//! management. The RHI will be implemented by at least Vulkan and Direct3D 12. I'd like to eventually also support
//! Metal, but there's a lot to do before then

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

// Re-exports
pub use rhi_enums::*;
pub use rhi_structs::*;
pub use rhi_traits::*;

// Re-export entry points each supported API
pub use vulkan::vulkan_graphics_api::VulkanGraphicsApi;

mod use super::shaderpack;