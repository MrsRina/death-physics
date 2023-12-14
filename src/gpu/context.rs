use erupt::{
  vk,
  cstr,
  EntryLoader, InstanceLoader
};

use std::ffi::CString;
use std::sync::OnceLock;
use crate::vklog;

pub struct Context {
  pub vk_entry_loader: EntryLoader,
  pub instance: InstanceLoader,
  pub physical_device: vk::PhysicalDevice,
}

impl Context {
  pub fn new(app_name: &'static str, engine_name: &'static str) -> Context {
    let app_name = CString::new(app_name).unwrap();
    let engine_name = CString::new(engine_name).unwrap();

    let app_info = vk::ApplicationInfoBuilder::new()
      .application_name(&app_name)
      .engine_name(&engine_name)
      .engine_version(vk::API_VERSION_1_0)
      .api_version(vk::API_VERSION_1_0);

    let vk_layer_khronos_validation : *const i8 = cstr!("VK_LAYER_KHRONOS_validation");
    let enabled_layer_names = vec![vk_layer_khronos_validation];

    let instance_create_info = vk::InstanceCreateInfoBuilder::new()
      .application_info(&app_info)
      .enabled_layer_names(&enabled_layer_names);

    let vk_entry_loader = EntryLoader::new().unwrap();

    let instance = unsafe {
      InstanceLoader::new(&vk_entry_loader, &instance_create_info).expect("Failed to create Vulkan instance!")
    };

    let physical_device = Context::create_vk_physical_device(&instance);

    Context {
      vk_entry_loader: vk_entry_loader,
      instance: instance,
      physical_device: physical_device,
    }
  }

  pub fn create_vk_physical_device(instance: &InstanceLoader) -> vk::PhysicalDevice {
    let physical_devices = unsafe { instance.enumerate_physical_devices(None).unwrap() };
    let mut physical_device = vk::PhysicalDevice::default();
    let mut first_device = true;

    for gpu_physical_device in physical_devices {
      let properties = unsafe { instance.get_physical_device_properties(gpu_physical_device) };
      let device_name : String = properties.device_name.iter()
        .take_while(|&&chars| chars != 0x00i8)
        .map(|&chars| chars as u8 as char)
        .collect();

      vklog!(device_name);

      if first_device {
        physical_device = gpu_physical_device;
        first_device = false;
      }
    }

    physical_device
  }
}