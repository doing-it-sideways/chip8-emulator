/*
Unlike the rest of the project, which only took help for a couple lines and parts of the toml,
the majority of the wasm specific code is based on the tutorial from @aquova on github.
*/
use chip8::interpreter::*;
use wasm_bindgen::prelude::*;

mod graphics;
mod input;

#[wasm_bindgen]
pub struct Chip8Wasm {
    settings: setup::Settings,
}

#[wasm_bindgen]
impl Chip8Wasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            settings: setup::setup()
        }
    }

    #[wasm_bindgen]
    pub fn change_rom(&mut self) {
        todo!()
    }

    #[wasm_bindgen]
    pub fn start(&mut self, gctx: graphics::JSGraphicsCtx, ihandle: input::JSInput) -> Result<(), JsError> {
        match run(&self.settings, gctx, ihandle) {
            Ok(_) => Ok(()),
            Err(e) => Err(JsError::new(&e.to_string()))
        }
    }
}