use erupt::{
    vk,
    cstr,
    EntryLoader, InstanceLoader
};

use std::ffi::CString;
use std::sync::OnceLock;
use std::collections::HashSet;

use crate::bitwise;

pub struct Context {
    pub vk_entry_loader: EntryLoader,
    pub instance: InstanceLoader,
    pub physical_device: vk::PhysicalDevice,
    pub surface: vk::SurfaceKHR,
}

impl Context {
    pub fn new(app_name: &'static str, engine_name: &'static str, sdl_win : &sdl2::video::Window) -> Context {
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

        let surface = sdl_win.vulkan_create_surface();
        let physical_device = Context::get_physical_device(&instance.getins);
        let device = Context::create_vk_device(&instance, physical_device, &surface);

        Context {
            vk_entry_loader: vk_entry_loader,
            instance: instance,
            physical_device: physical_device,
            surface: surface,
        }
    }

    pub fn get_physical_device(instance: &InstanceLoader) -> vk::PhysicalDevice {
        let physical_devices = unsafe { instance.enumerate_physical_devices(None).unwrap() };
        let mut physical_device = vk::PhysicalDevice::default();
        let mut first_device = true;

        for gpu_physical_device in physical_devices {
            let properties = unsafe { instance.get_physical_device_properties(gpu_physical_device) };
            let device_name : String = properties.device_name.iter()
                .take_while(|&&chars| chars != 0x00i8)
                .map(|&chars| chars as u8 as char)
                .collect();

            println!("[GPU] {}", device_name);

            if first_device {
                physical_device = gpu_physical_device;
                first_device = false;
            }
        }

        physical_device
    }

    pub fn create_vk_device(instance: &InstanceLoader, physical_device: vk::PhysicalDevice, surface: &vk::SurfaceKHR) -> vk::Device {
        let device_queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(physical_device, None) };

        let mut graphics_queue_index = 69;
        let mut transfer_queue_index = 69;
        let mut presentation_queue_index = 69;
        let mut queue_counter = 0;

        for property in device_queue_family_properties.iter() {
            if bitwise!(property.queue_flags, vk::QueueFlags::GRAPHICS) && graphics_queue_index != 69 {
                graphics_queue_index = queue_counter;
            }

            if bitwise!(property.queue_flags, vk::QueueFlags::TRANSFER) && queue_counter > 0 {
                transfer_queue_index = queue_counter;
            }

            println!("oii queue quantos filhos vc tem? {}", property.queue_count);

            let is_presentation_supported = unsafe { instance.get_physical_device_surface_support_khr(physical_device, queue_counter, *surface).unwrap() };
            if is_presentation_supported {
                presentation_queue_index = property.queue_count;
            }

            queue_counter += 1;
        }

        println!("[GPU] Found queue family indices: Graphics Transfer Presentation [{}, {}, {}]", graphics_queue_index, transfer_queue_index, presentation_queue_index);

        let mut device_queue_create_info_list = Vec::new();
    
        let mut queues: HashSet::<u32> = HashSet::new();
        queues.insert(graphics_queue_index);
        queues.insert(transfer_queue_index);
    
        let len = queues.len() as f32;
        let priority: Vec<f32> = (1..=len as usize).map(|latency| latency as f32 / len).collect();
    
        for queue_indices in queues {
            let device_queue_create_info = vk::DeviceQueueCreateInfoBuilder::new()
                .queue_family_index(queue_indices)
                .queue_priorities(priority.as_slice());

            device_queue_create_info_list.push(device_queue_create_info);
        }

        let enabled_extensions: Vec<*const i8> = vec![vk::KHR_SWAPCHAIN_EXTENSION_NAME as *const i8];
        let device_create_info = vk::DeviceCreateInfoBuilder::new()
            .queue_create_infos(&device_queue_create_info_list)
            .enabled_extension_names(&enabled_extensions);

        unsafe {
            instance.create_device(physical_device, &device_create_info, None).unwrap()
        }
    }
}