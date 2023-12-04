extern crate sdl2;

mod io;
mod gpu;

use gpu::Host;

pub struct Application {
    screen_width: u32,
    screen_height: u32,
}

impl Application {
    pub fn new() -> Application {
        Application {screen_width: 800u32, screen_height: 600u32,}
    }

    pub fn init(&mut self) {
        self.screen_width = 800;
        self.screen_height = 600;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video_subsystem = sdl_context.video().unwrap();
    let sdl_window = sdl_video_subsystem.window("Death Physics", 800, 600).vulkan().build().unwrap();

    let mut app = Application::new();
    app.init();

    let mut host : Host = Host::default();

    unsafe {
        gpu::init_instance(sdl_window.vulkan_instance_extensions().unwrap());
        gpu::init_physical_device(&mut host);
    }

    let mut sdl_event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in sdl_event_pump.poll_iter() {
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