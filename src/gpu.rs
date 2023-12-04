use std::{sync::OnceLock};
use std::ffi::CString;
use std::collections::HashSet;

use crate::bitwise;

use erupt::{
    cstr,
    vk, EntryLoader, InstanceLoader
};

static VK_ENTRY: OnceLock<EntryLoader> = OnceLock::new();
static VK_INSTANCE: OnceLock<InstanceLoader> = OnceLock::new();
static VK_PHYSICAL_DEVICE: OnceLock<vk::PhysicalDevice> = OnceLock::new();

#[derive(Default)]
pub struct Host {
    pub graphics_queue: u32,
    pub transfer_queue: u32,
}

const LAYER_KHRONOS_VALIDATION: *const i8 = cstr!("VK_LAYER_KHRONOS_validation");

pub unsafe fn init_instance(vulkan_instance_extensions: Vec<&'static str>) {
    let app_name = CString::new("ga").unwrap();
    let engine_name = CString::new("oi").unwrap();

    let app_info = vk::ApplicationInfoBuilder::new()
        .application_name(&app_name)
        .application_version(vk::make_api_version(0, 1, 0, 0))
        .engine_name(&engine_name)
        .engine_version(vk::make_api_version(0, 1, 0, 0));

    let mut enabled_layer_names: Vec<*const i8> = Vec::new(); 
    enabled_layer_names.push(LAYER_KHRONOS_VALIDATION);

    let mut enabled_layer_names: Vec<*const i8> = enabled_layer_names.iter().map(|&&validations| validations.)
    enabled_layer_names.push(vk::KHR_SURFACE_EXTENSION_NAME);

    let create_info = vk::InstanceCreateInfoBuilder::new()
        .application_info(&app_info)
        .enabled_extension_names(vulkan_instance_extensions.as_slice())
        .enabled_layer_names(enabled_layer_names.as_slice());

    VK_ENTRY.get_or_init(||EntryLoader::new().unwrap());
    VK_INSTANCE.get_or_init(|| InstanceLoader::new(&VK_ENTRY.get().unwrap(), &create_info).expect("Failed to create Vulkan instance loader!"));
}

pub unsafe fn init_physical_device(host: &mut Host) {
    let vk_instance = VK_INSTANCE.get().unwrap();
    let physical_devices = vk_instance.enumerate_physical_devices(None).unwrap();
        
    let device_count = physical_devices.len() as u32;
    let mut first_device_reach: bool = false;

    println!("Number of physical devices: {}", device_count);
    for physical_device in physical_devices {
        let properties = vk_instance.get_physical_device_properties(physical_device);
        let device_name: String = properties.device_name.iter()
            .take_while(|&&cstr| cstr != 0x00i8)
            .map(|&it| it as u8 as char)
            .collect();
        
        if !first_device_reach {
            VK_PHYSICAL_DEVICE.get_or_init(|| physical_device);
        }

        println!("Device name: {} -> Chosen {}", device_name, !first_device_reach);
        first_device_reach = true;
    }

    let physical_device = VK_PHYSICAL_DEVICE.get().unwrap();
    let device_family_queues = vk_instance.get_physical_device_queue_family_properties(*physical_device, Some(22));

    let mut _graphics_found = false;
    let mut _transfer_found = false;

    for queue_properties in device_family_queues.iter().take_while(|q_p| q_p.queue_count != 0) {
        if bitwise!(queue_properties.queue_flags, vk::QueueFlags::GRAPHICS) && !_graphics_found {
            println!("Queue GRAPHICS support {}", queue_properties.queue_count);
            host.graphics_queue = queue_properties.queue_count;
            _graphics_found = true;
        }

        if bitwise!(queue_properties.queue_flags, vk::QueueFlags::TRANSFER) && !_transfer_found {
            println!("Queue TRANSFER support {}", queue_properties.queue_count);
            host.transfer_queue = queue_properties.queue_count;
            _transfer_found = queue_properties.queue_count > 1;
        }

        if bitwise!(queue_properties.queue_flags, vk::QueueFlags::PROTECTED) {
            println!("Queue PROTECTED support {}", queue_properties.queue_count);
        }
    }

    let mut device_queue_create_info_list = Vec::new();

    let mut queues: HashSet::<u32> = HashSet::new();
    queues.insert(host.transfer_queue);
    queues.insert(host.graphics_queue);

    let len = queues.len() as f32;
    let priority: Vec<f32> = (1..=len as usize).map(|latency| latency as f32 / len).collect();

    for queue_indices in queues {
        let device_queue_create_info = vk::DeviceQueueCreateInfoBuilder::new()
            .queue_family_index(queue_indices)
            .queue_priorities(priority.as_slice());

        device_queue_create_info_list.push(device_queue_create_info);
    }

    let enabled_extensions = vec![vk::KHR_SWAPCHAIN_EXTENSION_NAME];
    let enabled_layer_names = vec![LAYER_KHRONOS_VALIDATION]; 

    let device_queue_create_info = vk::DeviceCreateInfoBuilder::new()
        .queue_create_infos(&device_queue_create_info_list)
        .enabled_extension_names(&enabled_extensions)
        .enabled_layer_names(&enabled_layer_names);

    vk_instance.create_device(*physical_device, &device_queue_create_info, None)
        .expect("Failed to create Vulkan device from physical device!");
}