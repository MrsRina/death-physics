use erupt::{
  vk,
  cstr,
  EntryLoader, InstanceLoader
};

use std::ffi::CString;
use crate::vklog;

pub struct Context {
  instance: InstanceLoader,
  physical_device: vk::PhysicalDevice,
}

impl Context {
  pub fn new(app_name: CString, engine_name: CString) -> Context {
    let instance = Context::create_vk_instance(app_name, engine_name);
    let physical_device = Context::create_vk_physical_device(instance);

    Context {
      instance: instance,
    }
  }

  pub fn create_vk_instance(app_name: CString, engine_name: CString) -> InstanceLoader {
    let app_info = vk::ApplicationInfoBuilder::new()
      .application_name(&app_name)
      .engine_name(&engine_name)
      .engine_version(vk::API_VERSION_1_0)
      .api_version(vk::API_VERSION_1_0);

    let VK_LAYER_KHRONOS_VALIDATION : *const i8 = cstr!("VK_LAYER_KHRONOS_validation");
    let enabled_layer_names = vec![VK_LAYER_KHRONOS_VALIDATION];

    let instance_create_info = vk::InstanceCreateInfoBuilder::new()
      .application_info(&app_info)
      .enabled_layer_names(&enabled_layer_names);

    let entry = EntryLoader::new().unwrap();

    unsafe {
      entry.create_instance(&instance_create_info, None)
    }
  }

  pub fn create_vk_physical_device(instance: InstanceLoader) -> vk::PhysicalDevice {
    let physical_devices : Vec<vk::PhysicalDevice> = instance.enumerate_physical_devices(None).unwrap();

    for physical_device in physical_devices {
      let properties = instance.get_physical_device_properties(physical_device);
      vklog!("oi");
    }
  }
}