use wasm_bindgen::prelude::*;

use web_sys::{
    KeyboardEvent,
};

use chip8::interpreter::{
    ProgramStatus,
    input::*,
};

#[wasm_bindgen]
pub struct JSInput {
    input: u16,
    has_quit: bool,
}

#[wasm_bindgen]
impl JSInput {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { 
            input: 0,
            has_quit: false,
        }
    }

    #[wasm_bindgen]
    #[allow(non_snake_case)]
    pub fn keypressEvent(&mut self, event: KeyboardEvent, pressed: bool) {
        let key_button_mapping = |keycode: &str| -> Option<u16> {
            Some(match keycode {
                "1" => 0x1,
                "2" => 0x2,
                "3" => 0x3,
                "4" => 0xC,
                "q" => 0x4,
                "w" => 0x5,
                "e" => 0x6,
                "r" => 0xD,
                "a" => 0x7,
                "s" => 0x8,
                "d" => 0x9,
                "f" => 0xE,
                "z" => 0xA,
                "x" => 0x0,
                "c" => 0xB,
                "v" => 0xF,
                _ => return None,
            })
        };

        if let Some(key) = key_button_mapping(&event.key()) {
            if pressed {
                self.input |= 1 << key;
            }
            else {
                self.input &= !(1 << key);
            }
        }
    }
}

impl InputHandler for JSInput {
    fn handle(&mut self, cur_input: &mut u16) -> ProgramStatus {
        *cur_input = self.input;

        if self.has_quit {
            ProgramStatus::Quit
        }
        else {
            ProgramStatus::Ok
        }
    }
}
