use std::{
    error::Error,
};

use chip8::interpreter::*;

mod graphics;
mod input;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_ctx = sdl3::init()?;

    let settings = setup::setup();

    let gctx = graphics::SDLGraphicsCtx::new(&sdl_ctx, settings.window_scale)?;

    let ihandle = input::SDLInput::new(sdl_ctx.event_pump()?);
    
    run(&settings, gctx, ihandle).map(|_ok| ())
}

