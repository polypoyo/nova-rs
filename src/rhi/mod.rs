//! Nova's Render Hardware Interface
//! 
//! This is an abstraction over Nova's supported APIs which presents an interface that was explicitly tailored to Nova.
//! This interface also hides some of the less tasteful parts of the supported APIs, such as the explicit memory 
//! management. The RHI will be implemented by at least Vulkan and Direct3D 12. I'd like to eventually also support 
//! Metal, but there's a lot to do before then

mod rhi;

// Re-exports
pub use rhi::*;
