use super::super::GraphicsApi;

use super::vulkan_physical_device::{self, VulkanPhysicalDevice};
use ash::{
    version::{EntryV1_0, InstanceV1_0},
    vk,
};
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

use crate::rhi::PhysicalDevice;
use ash::extensions::ext::DebugReport;
use log::debug;

unsafe extern "system" fn vulkan_debug_callback(
    _: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    debug!("{:?}", CStr::from_ptr(p_message));
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
    pub fn new(
        application_name: String,
        application_version: (u32, u32, u32),
    ) -> Result<VulkanGraphicsApi, VulkanGraphicsApiCreationError> {
        let layer_names = if cfg!(debug_assertions) {
            [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()]
        } else {
            []
        };

        let layer_names_raw = layer_names.iter().map(|n| n.as_ptr()).collect();

        let extension_names_raw = vulkan_physical_device::get_needed_extensions();

        let application_info = vk::ApplicationInfo::builder()
            .application_name(&application_name.into())
            .application_version(ash::vk_make_version!(
                application_version.0,
                application_version.1,
                application_version.2
            ))
            .engine_name(CString::new("Nova Renderer").as_c_str())
            .engine_version(ash::vk_make_version!(0, 1, 0))
            .api_version(ash::vk_make_version!(1, 1, 0))
            .build();

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&layer_names_raw)
            .enabled_extension_names(&extension_names_raw)
            .build();

        let entry = {
            let entry = ash::Entry::new();
            if entry.is_err() {
                return Err(VulkanGraphicsApiCreationError::LoadingError(
                    [entry.err().unwrap().0].to_vec(),
                ));
            }

            entry.unwrap()
        };

        let instance = {
            let instance = unsafe { entry.create_instance(&create_info, None) };
            if instance.is_err() {
                return match instance.err().unwrap() {
                    ash::InstanceError::LoadError(errors) => {
                        return Err(VulkanGraphicsApiCreationError::LoadingError(
                            errors.iter().map(|r| String::from(r)).collect(),
                        ));
                    }
                    ash::InstanceError::VkError(result) => {
                        return Err(VulkanGraphicsApiCreationError::VkFailedResult(result));
                    }
                };
            }

            instance.unwrap()
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
            let debug_callback = unsafe { debug_report_loader.create_debug_report_callback(&debug_info, None) };
            if debug_callback.is_err() {
                return Err(VulkanGraphicsApiCreationError::VkFailedResult(
                    debug_callback.err().unwrap(),
                ));
            }

            Some(debug_callback.unwrap())
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
        let mut devices = {
            let devices = unsafe { self.instance.enumerate_physical_devices() };
            if devices.is_err() {
                // TODO: The current trait doesn't allow us to return an error, what to do?
                return Vec::new();
            }

            devices
                .unwrap()
                .iter()
                .map(|d| VulkanPhysicalDevice::new(self.instance.handle(), *d))
                .filter(|d| d.can_be_used_by_nova())
                .collect()
        };

        devices
    }
}
