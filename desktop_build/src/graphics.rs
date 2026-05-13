use sdl3::{ 
    pixels::PixelFormat,
    render::{
        Texture,
        TextureAccess,
        WindowCanvas,
    },
    sys::{
        error::SDL_GetError,
        render,
        surface
    }
};

use std::error::Error;

use chip8::interpreter::{
    error::InterpreterErr,
    ProgramStatus,
    PixelBits,
    graphics::*,
};

pub struct SDLGraphicsCtx {
    canvas: WindowCanvas,
    screen_tex: Texture,
    pixel_buf: [u8; (WIDTH * HEIGHT * 4) as usize]
}

impl SDLGraphicsCtx {
    pub fn new(ctx: &sdl3::Sdl, window_scale: u8) -> Result<Self, Box<dyn Error>> {
        let vid_subsys = ctx.video()?;

        let window_scale = window_scale as u32;
        let window = vid_subsys.window("Chip-8 Interpreter",
                                        WIDTH * window_scale,
                                        HEIGHT * window_scale)
            .opengl()
            .position_centered()
            .build()?;

        let canvas = window.into_canvas();

        let screen_tex = canvas.texture_creator().create_texture(PixelFormat::RGBA8888, 
                                                                 TextureAccess::Streaming,
                                                                 WIDTH, HEIGHT)?;

        unsafe {
            if !render::SDL_SetTextureScaleMode(screen_tex.raw(), surface::SDL_SCALEMODE_NEAREST) {
                let err_str = std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy().into_owned();
                return Err(Box::new(InterpreterErr::APIError(err_str)));
            }
        }

        Ok(Self {
            canvas,
            screen_tex,
            pixel_buf: [0; (WIDTH * HEIGHT * 4) as usize]
        })
    }
}

impl Graphics for SDLGraphicsCtx {
    fn draw(&mut self, pixels: &PixelBits) -> Result<ProgramStatus, Box<dyn Error>> {
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

        self.canvas.present();

        Ok(ProgramStatus::Ok)
    }
}
