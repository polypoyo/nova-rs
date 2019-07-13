use super::settings_traits::*;
use crate::rhi::*;
pub struct Semver {
    major: u32,
    minor: u32,
    patch: u32,
}

/*
 * \brief All options to turn on debugging functionality
 */
pub struct NovaSettings {
    /*
     * \brief All options to turn on debugging functionality
     */
    debug: DebugOptions,
    /*
     * \brief Settings that Nova can change, but which are still stored in a config
     */
    cache: CacheOptions,
    /*
     * \brief Options about the window that Nova will live in
     */
    window: WindowOptions,
    /*
     * \brief Options that are specific to Nova's Vulkan rendering backend
     */
    vulkan: VulkanOptions,
    /*
     * \brief Options that are specific to Nova's DirectX 12 backend
     */
    dx12: Dx12Options,
    /*
     * \brief The rendering API to use
     *
     * DirectX 12 is only supported on Windows 10. On other platforms Vulkan will be used, regardless of what you've chosen
     */
    api: GraphicsAPI,
    /*
     * \brief Information about the system we're running on
     */
    system_info: SystemInfo,

    max_in_flight_frames: u32,

    /*
     * \brief Settings for how Nova should allocate vertex memory
     */
    vertex_memory_settings: BlockAllocatorSettings,

    /*
     * \brief Settings for how Nova should allocate index memory
     */
    index_memory_settings: BlockAllocatorSettings,

    config_change_listeners: Vec<ConfigListener>,
}

/*
 * \brief Options for configuring the way mesh memory is allocated
 *
 * Nova tries to be clever and optimize how it draws meshes with indirect rendering. It shoves everything into
 * a handful of giant buffers, to facilitate indirect rendering. These options are how you configure that
 */
pub struct BlockAllocatorSettings {
    /*
     * \brief The total amount of memory that can be used
     *
     * This must be a whole-number multiple of `new_buffer_size`
     */
    max_total_allocation: u32,

    /*
     * \brief The size of one buffer
     *
     * Nova doesn't allocate `max_total_allocation` memory initially. It only allocates a single buffer of
     * `new_buffer_size` size, then allocates new buffers as needed
     *
     * This number must be a whole-number multiple of `buffer_part_size`
     */
    new_buffer_size: u32,

    /*
     * \brief The size of one allocation from a buffer
     *
     * Nova gives meshes one or more allocations from a given buffer.
     */
    buffer_part_size: u32,
}

struct RenderDocOptions {
    /*
     * \brief If true, Nova will look for RenderDoc on your computer and will try to load it, letting you
     * debug your shaderpack without leaving Nova
     */
    enabled: bool,

    /*
     * \brief The path to `renderdoc.dll` on your filesystem
     */
    renderdoc_dll_path: str,

    /*
     * \brief The base path for RenderDoc captures
     */
    capture_path: str,
}
struct DebugOptions {
    /*
     * \brief If false, all debugging behavior is disabled, even if individual options are turned on
     */
    enabled: bool,

    /*
     * \breif Controls if the API-specific validation layers are enabled
     *
     * This should be enabled most of the time for Nova developers and almost never for shaderpack authors.
     * Nova developers need it on to debug their Vulkan or DX12 usage, while Nova should be robust enough that
     * errors that the validation layers would catch never happen in a shipping build
     */
    enable_validation_layers: bool,

    renderdoc: RenderDocOptions,
}

struct CacheOptions {
    /*
     * \brief The shaderpack that was most recently loaded
     *
     * Nova requires a shaderpack to render anything, so we need to know which one to load on application start
     */
    loaded_shaderpack: str,
}

struct WindowOptions {
    /*
     * \brief The title of the Window
     */
    title: str,

    /*
     * \brief The width of the window
     */
    width: u32,

    /*
     * \brief The height of the window
     */
    height: u32,
}

struct VulkanOptions {
    /*
     * \brief The application name to pass to Vulkan
     */
    application_name: str,

    /*
     * \brief The application version to pass to Vulkan
     */
    application_version: Semver,
}

struct Dx12Options {}

struct SystemInfo {
    /*
     * \brief Whether we're on a Unified Memory Architecture
     */
    is_uma: bool,
}
