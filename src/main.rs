extern crate sdl2;

use erupt::{InstanceLoader};

pub mod gpu {
    use erupt::{vk, EntryLoader};
    use crate::InstanceLoader;
    use std::ffi::CString;

    pub fn init_instance() {
        let app_name = CString::new("ga").unwrap();
        let engine_name = CString::new("oi").unwrap();

        let app_info = vk::ApplicationInfoBuilder::new()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 0, 0))
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 1, 0, 0));

        let create_info = vk::InstanceCreateInfoBuilder::new().application_info(&app_info);
        let entry_loader = EntryLoader::new().unwrap();

        let instance = unsafe {
            InstanceLoader::new(&entry_loader, &create_info).expect("Failed to create instance loader!")
        };
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem.window("Death-Physics", 800, 600).position_centered().vulkan().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    gpu::init_instance();

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