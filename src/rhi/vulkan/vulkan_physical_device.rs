use super::super::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use ash::{extensions::khr::Swapchain, vk};

pub struct VulkanPhysicalDevice {
    instance: vk::Instance,

    phys_device: vk::PhysicalDevice,
}

impl VulkanPhysicalDevice {
    fn supports_needed_extensions() -> bool {
        let available_extensions = match instance.enumerate_device_extension_properties(phys_device) {
            Ok(extensions) => extensions,
            Err(_) => Vec::new(),
        };

        let mut needed_extensions = get_needed_extensions()

        for ext in available_extensions {
            needed_extensions.remove(etx);
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
        let props = instance.get_physical_device_properties(phys_device);

        if !supports_needed_extensions() {
            return false;
        }

        true
    }

    fn create_logical_device(&self) -> Result<Self::Device, DeviceCreationError> {
        unimplemented!()
    }
}

#[cfg(all(unix, not(target_os = "android")))]
fn get_needed_extensions() -> Vec<*const i8> {
    vec![
        vk::Surface::name().as_ptr(),
        vk::XlibSurface::name().as_ptr(),
        vk::DebugReport::name().as_ptr()
    ]
}

#[cfg(windows)]
fn get_needed_extensions() -> Vec<*const i8> {
    vec![
        vk::Surface::name().as_ptr(),
        vk::Win32Surface::name().as_ptr(),
        vk::DebugReport::name().as_ptr()
    ]
}
