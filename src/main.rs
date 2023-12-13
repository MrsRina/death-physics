extern crate sdl2;

use std::ffi::CString;

pub mod utility;

use crate::gpu::context::Context;
pub mod gpu;

pub struct Application {
  window_width: u32,
  window_height: u32,
  sdl: sdl2::Sdl,
  sdl_video_subsystem: sdl2::VideoSubsystem,
  sdl_window: sdl2::video::Window,
  vk_context: Context,
}

impl Application {
  pub fn init(&mut self) {
      self.window_width = 800;
      self.window_height = 600;
  }

  pub fn mainloop(&mut self) {
    let mut sdl_event_pump = self.sdl.event_pump().unwrap();

    'running: loop {
      for sdl_event in sdl_event_pump.poll_event() {
        match sdl_event {
          sdl2::event::Event::Quit { .. } => {
            break 'running;
          },
          _ => {}
        }

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60u32));
      }
    }
  }
}

fn main() {
  let sdl = sdl2::init().unwrap();
  let sdl_video_subsystem = sdl.video().unwrap();
  let sdl_window = sdl_video_subsystem.window("Death Physics", 800, 600).vulkan().build().unwrap();

  let vk_context = Context::new(CString::new("death physics vk").unwrap(), CString::new("death physics engine").unwrap());

  let mut app = Application {
    window_width: 800, 
    window_height: 600,
    sdl: sdl,
    sdl_video_subsystem: sdl_video_subsystem,
    sdl_window: sdl_window,
    vk_context: vk_context,
  };

  app.init();
  app.mainloop();
}