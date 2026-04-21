use super::{
    error::InterpreterErr,
    ProgramStatus,
};

use sdl3::{
    keyboard::Keycode,
    EventPump,
};

pub fn update(event_pump: &mut EventPump, cur_input: &mut u16) -> ProgramStatus {
    use sdl3::event::Event;

    let key_button_mapping = |keycode: Keycode| -> Option<u16> {
        Some(match keycode {
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
            _ => return None,
        })
    };

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return ProgramStatus::Quit;
            },
            Event::KeyDown { keycode: Some(keycode), .. } => {
                if let Some(index) = key_button_mapping(keycode) {
                    *cur_input |= 1 << index;
                }
            },
            Event::KeyUp { keycode: Some(keycode), .. } => {
                if let Some(index) = key_button_mapping(keycode) {
                    *cur_input &= !(1 << index);
                }
            },
            _ => (),
        }
    }
    
    ProgramStatus::Ok
}
