use crate::rhi::vulkan::vulkan_physical_device;
use crate::rhi::vulkan::vulkan_physical_device::VulkanPhysicalDevice;
use crate::rhi::*;

use ash::extensions::ext::DebugReport;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;
use log::debug;
use std::ffi;
use std::os::raw;

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const raw::c_char,
    p_message: *const raw::c_char,
    _: *mut raw::c_void,
) -> u32 {
    debug!("{:?}", ffi::CStr::from_ptr(p_message));
    vk::FALSE
}

#[derive(Debug)]
pub enum VulkanGraphicsApiCreationError {
    VkFailedResult(vk::Result),
    LoadingError(Vec<String>),
}

pub struct VulkanGraphicsApi {
    instance: ash::Instance,
    debug_callback: Option<vk::DebugReportCallbackEXT>,
    entry: ash::Entry,
}

impl VulkanGraphicsApi {
    pub fn get_layer_names() -> Vec<*const u8> {
        (if cfg!(debug_assertions) {
            [ffi::CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()]
        } else {
            []
        })
        .iter()
        .map(|n| n.as_ptr())
        .collect()
    }

    pub fn new(
        application_name: String,
        application_version: (u32, u32, u32),
    ) -> Result<VulkanGraphicsApi, VulkanGraphicsApiCreationError> {
        let layer_names_raw = VulkanGraphicsApi::get_layer_names().as_slice();

        let extension_names_raw = vulkan_physical_device::get_needed_extensions();

        let application_info = vk::ApplicationInfo::builder()
            .application_name(&application_name.into())
            .application_version(ash::vk_make_version!(
                application_version.0,
                application_version.1,
                application_version.2
            ))
            .engine_name(ffi::CString::new("Nova Renderer").as_c_str())
            .engine_version(ash::vk_make_version!(0, 1, 0))
            .api_version(ash::vk_make_version!(1, 1, 0))
            .build();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layer_names_raw)
            .enabled_extension_names(&extension_names_raw)
            .build();

        let entry = match ash::Entry::new() {
            Err(error) => {
                return Err(VulkanGraphicsApiCreationError::LoadingError(
                    [error.unwrap().0].to_vec(),
                ));
            }
            Ok(v) => v,
        };

        let instance = match unsafe { entry.create_instance(&create_info, None) } {
            Err(error) => {
                return match error {
                    ash::InstanceError::LoadError(errors) => Err(VulkanGraphicsApiCreationError::LoadingError(
                        errors.iter().map(|raw| String::from(raw)).collect(),
                    )),
                    ash::InstanceError::VkError(result) => Err(VulkanGraphicsApiCreationError::VkFailedResult(result)),
                };
            }
            Ok(v) => v,
        };

        let debug_callback = if cfg!(debug_assertions) {
            let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
                .flags(
                    vk::DebugReportFlagsEXT::ERROR
                        | vk::DebugReportFlagsEXT::WARNING
                        | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
                        | vk::DebugReportFlagsEXT::INFORMATION
                        | vk::DebugReportFlagsEXT::DEBUG,
                )
                .pfn_callback(Some(vulkan_debug_callback));

            let debug_report_loader = DebugReport::new(&entry, &instance);
            match unsafe { debug_report_loader.create_debug_report_callback(&debug_info, None) } {
                Err(error) => return Err(VulkanGraphicsApiCreationError::VkFailedResult(error)),
                Ok(v) => Some(v),
            }
        } else {
            None
        };

        Ok(VulkanGraphicsApi {
            instance,
            debug_callback,
            entry,
        })
    }
}

impl GraphicsApi for VulkanGraphicsApi {
    type PhysicalDevice = VulkanPhysicalDevice;

    fn get_adapters(&self) -> Vec<VulkanPhysicalDevice> {
        let devices = unsafe { self.instance.enumerate_physical_devices() };
        if devices.is_err() {
            // TODO: The current trait doesn't allow us to return an error, what to do?
            return Vec::new();
        }

        devices
            .unwrap()
            .iter()
            .map(|d| VulkanPhysicalDevice::new(self.instance, *d))
            .filter(|d| d.can_be_used_by_nova())
            .collect()
    }
}
