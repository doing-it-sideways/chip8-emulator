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
    settings: setup::Settings, // TODO: remove
    interpreter: Chip8,
    quit: bool
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            settings: setup::setup(), // TODO: remove
            interpreter: Chip8::new(),
            quit: false
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
    pub fn tick(&mut self,
                gctx: &mut graphics::JSGraphicsCtx, ihandle: &mut input::JSInput)
                -> Result<(), JsError>
    {
        // idk if this can even get set from the web from anything besides a halt.
        // will i test it? nope.
        if self.quit {
            return Ok(())
        }

        match self.interpreter.tick(gctx, ihandle) {
            Ok(ProgramStatus::Quit) => self.quit = true,
            Err(jserr) => return Err(JsError::new(&jserr.to_string())),
            _ => (),
        }

        Ok(())
    }
}