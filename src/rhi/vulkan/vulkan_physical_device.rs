use super::super::{DeviceCreationError, PhysicalDevice, PhysicalDeviceProperties};

use ash::{
    extensions::{ext::DebugReport, khr::Swapchain},
    vk,
};

#[cfg(all(unix, not(target_os = "android")))]
use ash::extensions::khr::XlibSurface;

#[cfg(windows)]
use ash::extensions::khr::Win32Surface;

use crate::rhi::{
    vulkan::vulkan_device::VulkanDevice, PhysicalDeviceManufacturer, PhysicalDeviceType, VulkanGraphicsApi,
};
use ash::version::{InstanceV1_0, InstanceV1_1};

pub struct VulkanPhysicalDevice {
    instance: ash::Instance,

    phys_device: vk::PhysicalDevice,

    graphics_queue_family_index: usize,
    compute_queue_family_index: usize,
    transfer_queue_family_index: usize,
}

impl VulkanPhysicalDevice {
    pub fn new(instance: ash::Instance, phys_device: vk::PhysicalDevice) -> VulkanPhysicalDevice {
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
        let queue_family_props: Vec<vk::QueueFamilyProperties> = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(self.phys_device)
        };

        for (index, props) in queue_family_props.iter().enumerate() {
            // TODO: At this stage we can't check if a surface is supported since we didn't create one yet
            // let supports_present = self.instance.get_physical_device_queue_family_properties()
            // if !supports_present {
            // continue;
            // }

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
        let available_extensions =
            match unsafe { self.instance.enumerate_device_extension_properties(self.phys_device) } {
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
        let properties: vk::PhysicalDeviceProperties =
            unsafe { self.instance.get_physical_device_properties(self.phys_device) };
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
        let graphics_queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(self.graphics_queue_family_index as u32)
            .queue_priorities(&[1.0f32])
            .build();

        let transfer_queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(self.transfer_queue_family_index as u32)
            .queue_priorities(&[1.0f32])
            .build();

        let queue_create_infos = if self.compute_queue_family_index != std::usize::MAX {
            let compute_queue_create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(self.compute_queue_family_index as u32)
                .queue_priorities(&[1.0f32])
                .build();

            [
                graphics_queue_create_info,
                transfer_queue_create_info,
                compute_queue_create_info,
            ]
        } else {
            [graphics_queue_create_info, transfer_queue_create_info]
        };

        let physical_device_features = vk::PhysicalDeviceFeatures::builder()
            .geometry_shader(true)
            .tessellation_shader(true)
            .sampler_anisotropy(true)
            .build();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&physical_device_features)
            .enabled_extension_names(&[Swapchain::name()])
            .enabled_layer_names(VulkanGraphicsApi::get_layer_names().as_slice())
            .build();

        let device: Result<ash::Device, vk::Result> =
            unsafe { self.instance.create_device(self.phys_device, &device_create_info, None) };
        if device.is_err() {
            Err(DeviceCreationError::Failed)
        } else {
            Ok(VulkanDevice::new(self.instance.clone(), device.unwrap()))
        }
    }

    fn get_free_memory(&self) -> u64 {
        // TODO: This just return all available memory, vulkan does not provide a way to query free memory
        //       on windows this could be done using DXGI (also works with vulkan according to stackoverflow),
        //       for linux a way has yet to be found
        let properties: vk::PhysicalDeviceMemoryProperties =
            self.instance.get_physcial_device_memory_properties(self.phys_device);
        properties
            .memory_heaps
            .iter()
            .filter(|h| h.flags & vk::MemoryHeapFlags::DEVICE_LOCAL)
            .map(|h| h.size)
            .sum()
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
