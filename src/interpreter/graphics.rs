use std::{ 
    error::Error,
};

use sdl3::{ 
    event::Event, // TODO: move to input
    keyboard::Keycode, // TODO: move to input
    pixels::{Color, PixelFormat},
    render::{
        Texture,
        TextureAccess,
    },
};

use super::ProgramStatus;

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub struct GraphicsCtx {
    canvas: sdl3::render::WindowCanvas,
    screen_tex: sdl3::render::Texture,
    event_pump: sdl3::EventPump,
    pixel_buf: [u8; (WIDTH * HEIGHT * 4) as usize]
}

impl GraphicsCtx {
    pub fn init(ctx: &sdl3::Sdl, window_scale: u8) -> Result<Self, Box<dyn Error>> {
        let vid_subsys = ctx.video()?;

        let window_scale = window_scale as u32;
        let window = vid_subsys.window("Chip-8 Interpreter",
                                        WIDTH * window_scale,
                                        HEIGHT * window_scale)
            .opengl()
            .position_centered()
            .build()?;

        let canvas  = window.into_canvas();

        let event_pump = ctx.event_pump()?;

        let screen_tex = canvas.texture_creator().create_texture(PixelFormat::RGBA8888, 
                                                              TextureAccess::Streaming,
                                                              WIDTH, HEIGHT)?;
    
        Ok(GraphicsCtx {
            canvas,
            screen_tex,
            event_pump,
            pixel_buf: [0u8; (WIDTH * HEIGHT * 4) as usize],
        })
    }

    pub fn draw(&mut self, pixels: &super::PixelBits) -> Result<ProgramStatus, Box<dyn Error>> {
        self.canvas.clear();

        for y in 0..HEIGHT as u8 {
            for x in 0..WIDTH as u8 {
                let index = 4 * (y as usize * WIDTH as usize + x as usize);
                let color = if pixels.get(x, y) == 0 { 0x00 } else { 0xFF };

                for i in 0..4 {
                    self.pixel_buf[index + i] = color;
                }
            }
        }

        self.screen_tex.update(None, &self.pixel_buf, 4 * WIDTH as usize)?;
        self.canvas.copy(&self.screen_tex, None, None)?;

        // TODO: move to input
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(ProgramStatus::Quit);
                },
                _ => {}
            }
        }

        self.canvas.present();

        Ok(ProgramStatus::Ok)
    }
}
