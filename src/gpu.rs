use core::ptr;
use std::ffi::CString;

#[macro_use]
use lazy_static::lazy_static;

extern crate ash;
use ash::version::{InstanceV1_0, EntryV1_0};

lazy_static! {
    static ref VULKAN_INSTANCE: ash::Instance = {
        let entry = ash::Entry::new().expect("Failed to load Vulkan entry points!");
        let app_name = CString::new("Dea").unwrap();
        let engine_name = CString::new("Gay").unwrap();

        let app_info = ash::version::VkApplicationInfo {
            p_application_name: app_name.as_ptr(),
            s_type: ash::version::VkStructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            application_version: ash::version::VK_MAKE_VERSION(1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: ash::version::VK_MAKE_VERSION(1, 0, 0),
            api_version: ash::version::VK_API_VERSION_1_0,
            ..Default::default()
        };

        let create_info = ash::version::VkInstanceCreateInfo {
            s_type: ash::version::VkStructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            ..Default::default()
        };

        unsafe {
            entry.create_instance(&create_info, None).expect("Failed to create Vulkan instance!");
        }
    };
}