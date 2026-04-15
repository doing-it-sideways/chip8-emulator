use super::{
    error::InterpreterErr,
    ProgramStatus,
};

use sdl3::{
    keyboard::Keycode,
    EventPump,
};

pub fn update(event_pump: &mut EventPump) -> (u16, ProgramStatus) {
    use sdl3::event::Event;
    let mut input = 0;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return (0, ProgramStatus::Quit);
            },
            Event::KeyDown { keycode: Some(keycode), .. } => {
                let index = match keycode {
                    Keycode::_1 => 0x1,
                    Keycode::_2 => 0x2,
                    Keycode::_3 => 0x3,
                    Keycode::_4 => 0xC,
                    Keycode::Q  => 0x4,
                    Keycode::W  => 0x5,
                    Keycode::E  => 0x6,
                    Keycode::R  => 0xD,
                    Keycode::A  => 0x7,
                    Keycode::S  => 0x8,
                    Keycode::D  => 0x9,
                    Keycode::F  => 0xE,
                    Keycode::Z  => 0xA,
                    Keycode::X  => 0x0,
                    Keycode::C  => 0xB,
                    Keycode::V  => 0xF,
                    _ => continue,
                };

                input |= 1 << index;
            }
            _ => (),
        }
    }
    
    (input, ProgramStatus::Ok)
}
