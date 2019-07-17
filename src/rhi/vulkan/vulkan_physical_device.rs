use super::super::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use ash::{
    extensions::{ext::DebugReport, khr::Swapchain},
    vk,
};

#[cfg(all(unix, not(target_os = "android")))]
use ash::extensions::khr::XlibSurface;

#[cfg(windows)]
use ash::extensions::khr::Win32Surface;

use crate::rhi::{vulkan::vulkan_device::VulkanDevice, PhysicalDeviceManufacturer, PhysicalDeviceType};

pub struct VulkanPhysicalDevice {
    instance: vk::Instance,

    phys_device: vk::PhysicalDevice,

    graphics_queue_family_index: usize,
    compute_queue_family_index: usize,
    transfer_queue_family_index: usize,
}

impl VulkanPhysicalDevice {
    pub fn new(instance: vk::Instance, phys_device: vk::PhysicalDevice) -> VulkanPhysicalDevice {
        let mut dev = VulkanPhysicalDevice {
            instance,
            phys_device,
            graphics_queue_family_index: std::usize::MAX,
            compute_queue_family_index: std::usize::MAX,
            transfer_queue_family_index: std::usize::MAX,
        };
        dev.detect_queues();
        dev
    }

    fn detect_queues(&mut self) {
        let queue_family_props: Vec<vk::QueueFamilyProperties> = self
            .instance
            .get_physical_device_queue_family_properties(self.phys_device);

        for (index, props) in queue_family_props.iter().enumerate() {
            let supports_present = self.phys_device.get_physical_device_surface_support_khr();
            if !supports_present {
                continue;
            }

            if self.graphics_queue_family_index == std::usize::MAX
                && props.queue_flags & vk::QueueFlags::GRAPHICS != 0u32
            {
                self.graphics_queue_family_index = index
            }

            if self.compute_queue_family_index == std::usize::MAX && props.queue_flags & vk::QueueFlags::COMPUTE != 0u32
            {
                self.compute_queue_family_index = index
            }

            if self.transfer_queue_family_index == std::usize::MAX
                && props.queue_flags & vk::QueueFlags::TRANSFER != 0u32
            {
                self.transfer_queue_family_index = index
            }
        }
    }

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

    fn get_manufacturer(&self, properties: &vk::PhysicalDeviceProperties) -> PhysicalDeviceManufacturer {
        match properties.vendor_id {
            // see http://vulkan.gpuinfo.org/
            // and https://www.reddit.com/r/vulkan/comments/4ta9nj/is_there_a_comprehensive_list_of_the_names_and/
            //  (someone find a better link here than reddit)
            0x1002 => PhysicalDeviceManufacturer::AMD,
            0x10DE => PhysicalDeviceManufacturer::Nvidia,
            0x8086 => PhysicalDeviceManufacturer::Intel,
            _ => PhysicalDeviceManufacturer::Other,
        }
    }
}

impl PhysicalDevice for VulkanPhysicalDevice {
    type Device = VulkanDevice;

    fn get_properties(&self) -> PhysicalDeviceProperties {
        let properties: vk::PhysicalDeviceProperties = self.instance.get_physical_device_properties(self.phys_device);
        PhysicalDeviceProperties {
            manufacturer: self.get_manufacturer(&properties),
            device_id: properties.device_id,
            device_name: String::from(properties.device_name),
            device_type: match properties.device_type {
                vk::PhysicalDeviceType::INTEGRATED_GPU => PhysicalDeviceType::Integrated,
                vk::PhysicalDeviceType::DISCRETE_GPU => PhysicalDeviceType::Discreet,
                vk::PhysicalDeviceType::VIRTUAL_GPU => PhysicalDeviceType::Virtual,
                vk::PhysicalDeviceType::CPU => PhysicalDeviceType::CPU,
                vk::PhysicalDeviceType::OTHER => PhysicalDeviceType::Other,
            },
            max_color_attachments: properties.limits.max_color_attachments,
        }
    }

    fn can_be_used_by_nova(&self) -> bool {
        if !self.supports_needed_extensions() {
            false
        }

        self.graphics_queue_family_index != std::usize::MAX && self.transfer_queue_family_index != std::usize::MAX
    }

    fn create_logical_device(&self) -> Result<Self::Device, DeviceCreationError> {
        unimplemented!()
    }
}

#[cfg(all(unix, not(target_os = "android")))]
pub fn get_needed_extensions() -> Vec<*const u8> {
    vec![
        Swapchain::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}

#[cfg(windows)]
pub fn get_needed_extensions() -> Vec<*const u8> {
    vec![
        Swapchain::name().as_ptr(),
        Win32Surface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}
