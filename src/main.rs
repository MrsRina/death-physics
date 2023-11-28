extern crate sdl2;

use erupt::{EntryLoader, InstanceLoader};
use std::sync::OnceLock;

static VK_ENTRY: OnceLock<EntryLoader> = OnceLock::new();
static VK_INSTANCE: OnceLock<InstanceLoader> = OnceLock::new();

pub mod gpu {
    use erupt::{vk, EntryLoader, InstanceLoader};
    use std::ffi::CString;

    use crate::VK_INSTANCE;
    use crate::VK_ENTRY;

    pub fn init_instance() {
        let app_name = CString::new("ga").unwrap();
        let engine_name = CString::new("oi").unwrap();

        let app_info = vk::ApplicationInfoBuilder::new()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0));

        let create_info = vk::InstanceCreateInfoBuilder::new().application_info(&app_info);

        VK_ENTRY.get_or_init(||EntryLoader::new().unwrap());

        unsafe {
            VK_INSTANCE.get_or_init(
                || InstanceLoader::new(&VK_ENTRY.get().unwrap(), &create_info).expect("Failed to create Vulkan instance loader!")
            )
        };
    }

    pub fn init_physical_device() {
        let vk_instance = VK_INSTANCE.get().unwrap();
        let physical_devices = unsafe { vk_instance.enumerate_physical_devices(None) }.unwrap();
        let device_count = physical_devices.len() as u32;

        println!("Number of physical devices: {}", device_count);
        for physical_device in physical_devices {
            let properties = unsafe { vk_instance.get_physical_device_properties(physical_device) };
            println!("Device name: {}", String::from_utf8_lossy(properties.device_name as [u8]));
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem.window("Death-Physics", 800, 600).position_centered().vulkan().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    gpu::init_instance();
    gpu::init_physical_device();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}