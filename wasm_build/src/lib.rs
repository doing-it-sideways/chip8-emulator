/*
Unlike the rest of the project, which only took help for a couple lines and parts of the toml,
the majority of the wasm specific code is based on the tutorial from @aquova on github.
*/
use chip8::interpreter::*;
use wasm_bindgen::prelude::*;

use js_sys::{
    Uint8Array,
};

mod graphics;
mod input;

#[wasm_bindgen]
pub struct Chip8Wasm {
    settings: setup::Settings,
    interpreter: Chip8,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            settings: setup::setup(),
            interpreter: Chip8::new(),
        }
    }

    #[wasm_bindgen]
    pub fn reload(&mut self, mode: &str, rom: Uint8Array) {
        use setup::InterpreterMode::*;
        let mode = match mode {
            "COSMAC" => COSMAC,
            "SUPERCHIP" => SUPERCHIP,
            "Octo" | _ => Octo,
        };

        self.interpreter.reload(mode, Some(&rom.to_vec()));
    }

    #[wasm_bindgen]
    pub fn start(&mut self, mut gctx: graphics::JSGraphicsCtx, mut ihandle: input::JSInput) -> Result<(), JsError> {
        'runloop: loop {
            match self.interpreter.tick(&mut gctx, &mut ihandle) {
                Ok(ProgramStatus::Quit) => break 'runloop,
                Err(jserr) => return Err(JsError::new(&jserr.to_string())),
                _ => (),
            }

            //std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / 60.0));
        }

        Ok(())
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        todo!()
    }
}