use std::{ 
    error::Error,
    time::Duration,
};

use sdl3::{ 
    pixels::Color,
    event::Event,
    keyboard::Keycode,
};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub fn init(window_scale: u8) -> Result<(), Box<dyn Error>> {
    let ctx = sdl3::init()?;
    let vid_subsys = ctx.video()?;

    let window = vid_subsys.window("Chip-8 Interpreter", WIDTH * window_scale as u32, HEIGHT * window_scale as u32)
        .opengl()
        .position_centered()
        .build()?;

    let mut canvas  = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = ctx.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}

