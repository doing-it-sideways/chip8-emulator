use std::{ 
    error::Error,
};

use sdl3::{ 
    event::Event, // TODO: move to input
    keyboard::Keycode, // TODO: move to input
    pixels::{
        Color,
    },
    render::{
        Texture,
        TextureAccess,
    }
};

pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub struct GraphicsCtx {
    canvas: sdl3::render::WindowCanvas,
    texture: sdl3::render::Texture,
    event_pump: sdl3::EventPump,
}

// TODO: move to input
pub struct QuitEvent;

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

        let mut canvas  = window.into_canvas();

        let event_pump = ctx.event_pump()?;

        let texture = canvas.texture_creator().create_texture(None, 
                                                              TextureAccess::Streaming,
                                                              WIDTH, HEIGHT)?;
        
        //canvas.copy(&texture, None, None);
    
        Ok(GraphicsCtx {
            canvas,
            texture,
            event_pump
        })
    }

    pub fn draw(&mut self, pixels: &super::PixelBits) -> Result<(), QuitEvent> {
        let mut i = 0;
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();
        self.canvas.texture_creator();
        

        // TODO: move to input
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err(QuitEvent);
                },
                _ => {}
            }
        }

        self.canvas.present();

        Ok(())
    }
}
