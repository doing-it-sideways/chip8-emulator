use std::{ 
    error::Error,
    time::Duration,
};

use sdl3::{ 
    event::Event,
    keyboard::Keycode,
    pixels::Color,
};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub struct GraphicsCtx {
    ctx: sdl3::Sdl,
    canvas: sdl3::render::WindowCanvas,
    event_pump: sdl3::EventPump,
}

pub struct QuitEvent;

impl GraphicsCtx {
    pub fn init(window_scale: u8) -> Result<Self, Box<dyn Error>> {
        let ctx = sdl3::init()?;
        let vid_subsys = ctx.video()?;

        let window = vid_subsys.window("Chip-8 Interpreter",
                                        WIDTH * window_scale as u32,
                                        HEIGHT * window_scale as u32)
            .opengl()
            .position_centered()
            .build()?;

        let mut canvas  = window.into_canvas();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = ctx.event_pump()?;
    
        Ok(GraphicsCtx {
            ctx,
            canvas,
            event_pump
        })
    }

    pub fn draw(&mut self) -> Result<(), QuitEvent> {
        let mut i = 0;
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(QuitEvent);
                },
                _ => {}
            }
        }

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        Ok(())
    }
}
