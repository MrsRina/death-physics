use erupt::{
  vk,
  cstr,
  EntryLoader, InstanceLoader
};

use std::ffi::CString;

pub struct Context {
  entry: EntryLoader,
  instance: InstanceLoader,
  physical_device: vk::PhysicalDevice,
}

impl Context {
  pub fn new(app_name: CString, engine_name: CString) -> Context {
    let app_info = vk::ApplicationInfoBuilder::new()
      .application_name(&app_name)
      .engine_name(&engine_name)
      .engine_version(vk::API_VERSION_1_0)
      .api_version(vk::API_VERSION_1_0);

    let instance_create_info = vk::InstanceCreateInfoBuilder::new()
      .application_info(&app_info)
      .enabled_layer_names(enabled_layer_names)
      .enabled_extension_names(enabled_extension_names);

    let entry = EntryLoader::new().unwrap();
    let instance = unsafe { entry.create_instance(&instance_create_info) };

    Context {
      entry: entry,
      instance: instance,
    }
  }

  pub fn create_vk_instance() {

  }
}