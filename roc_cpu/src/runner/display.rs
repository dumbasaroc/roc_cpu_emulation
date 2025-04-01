use sdl3::{pixels::Color, rect::Rect, *};

pub const DISPLAY_WIDTH: u32 = 40;
pub const DISPLAY_HEIGHT: u32 = 32;
const DISPLAY_SCALE: u32 = 20;

pub const DISPLAY_MEMORY_USAGE: u32 = 3 * DISPLAY_WIDTH * DISPLAY_HEIGHT;
pub const DISPLAY_MEMORY_START: usize = 0x8000;
pub const DISPLAY_MEMORY_END: usize = DISPLAY_MEMORY_START + DISPLAY_MEMORY_USAGE as usize;

#[allow(dead_code)]
pub struct RocCPUDisplay {
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: video::Window,
    event_pump: EventPump,
}

// User-level API

impl RocCPUDisplay {
    pub fn new() -> Self {
        
        let sdl = match sdl3::init() {
            Ok(sdl) => sdl,
            Err(err) => {
                panic!("SDL failed to initialize: {}", err);
            }
        };
        let video_subsystem = match sdl.video() {
            Ok(v) => v,
            Err(err) => {
                panic!("SDL Video failed to initialize: {}", err);
            }
        };

        let window = video_subsystem
            .window(
                "RocCPU EMU",
                DISPLAY_WIDTH * DISPLAY_SCALE,
                DISPLAY_HEIGHT * DISPLAY_SCALE
            )
            .borderless()
            .build()
            .unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Self {
            sdl,
            video_subsystem,
            window,
            event_pump
        }
    }

    pub fn render_current(&mut self, vmem: &[u8]) {
        for event in self.event_pump.poll_iter() {
            match event {
                _ => {}
            }
        }
        self.copy_vmem_to_window(vmem);
    }
}


// Private Methods

impl RocCPUDisplay {
    fn copy_vmem_to_window(&mut self, vmem: &[u8]) {
        let mut window_surface = match self.window.surface(&self.event_pump) {
            Ok(surf) => surf,
            Err(e) => {
                panic!("Could not get window surface: {}", e);
            }
        };

        for i in 0..DISPLAY_WIDTH {
            for j in 0..DISPLAY_HEIGHT {
                let idx = (i * 3 * DISPLAY_HEIGHT + j * 3) as usize;
                let color = Color::RGB(
                    vmem[idx],
                    vmem[idx + 1],
                    vmem[idx + 2]
                );

                window_surface.fill_rect(
                    Rect::new(
                        (i * DISPLAY_SCALE) as i32,
                        (j * DISPLAY_SCALE) as i32,
                        DISPLAY_SCALE,
                        DISPLAY_SCALE
                    ),
                    color
                ).unwrap();
            }
        }
        window_surface.finish().unwrap();
    }
}
