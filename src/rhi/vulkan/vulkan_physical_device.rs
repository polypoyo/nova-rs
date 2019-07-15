use super::super::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use ash::{
    extensions::{
        ext::DebugReport,
        khr::{Swapchain, Win32Surface, XlibSurface},
    },
    vk,
};

pub struct VulkanPhysicalDevice {
    instance: vk::Instance,

    phys_device: vk::PhysicalDevice,

    graphics_queue_family_index: u32,
    compute_queue_family_index: u32,
    copy_queue_family_index: u32,
}

impl VulkanPhysicalDevice {
    fn supports_needed_extensions(&self) -> bool {
        let available_extensions = match self.instance.enumerate_device_extension_properties(self.phys_device) {
            Ok(extensions) => extensions,
            Err(_) => Vec::new(),
        };

        let mut needed_extensions = get_needed_extensions();

        for ext in available_extensions {
            needed_extensions.remove(ext.extension_name);
        }

        needed_extensions.is_empty()
    }
}

impl PhysicalDevice for VulkanPhysicalDevice {
    type Device = ();

    fn get_properties(&self) -> PhysicalDeviceProperties {
        unimplemented!()
    }

    fn can_be_used_by_nova(&self) -> bool {
        if !self.supports_needed_extensions() {
            false
        }

        let queue_family_props: Vec<vk::QueueFamilyProperties> = self
            .instance
            .get_physical_device_queue_family_properties(self.phys_device);

        for props in queue_family_props {
            let supports_present = self.phys_device.get_physical_device_surface_support_khr();
            let supports_graphics = props.queue_flags | vk::QueueFlags::GRAPHICS;
        }

        true
    }

    fn create_logical_device(&self) -> Result<Self::Device, DeviceCreationError> {
        unimplemented!()
    }
}

#[cfg(all(unix, not(target_os = "android")))]
fn get_needed_extensions() -> Vec<*const u8> {
    vec![
        Swapchain::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}

#[cfg(windows)]
fn get_needed_extensions() -> Vec<*const u8> {
    vec![
        Swapchain::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}
