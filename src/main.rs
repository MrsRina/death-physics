extern crate sdl2;
mod gpu;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _window = video_subsystem.window("Death-Physics", 800, 600).position_centered().vulkan().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60))
    }
}