use std::error::Error;

use wasm_bindgen::{
    prelude::*,
    JsCast,
};

use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
};

use chip8::interpreter::{
    PixelBits,
    ProgramStatus,
    graphics::*,
};

#[wasm_bindgen]
pub struct JSGraphicsCtx {
    gctx: CanvasRenderingContext2d,
    scale: u8,
    color_fg: String,
    color_bg: String,
}

#[wasm_bindgen]
impl JSGraphicsCtx {
    #[wasm_bindgen(constructor)]
    pub fn new(scale: u8, color_fg: Option<String>, color_bg: Option<String>) -> Result<Self, JsError> {
        let doc = web_sys::window().ok_or_else(|| JsError::new("Invalid window"))?
                        .document().ok_or_else(|| JsError::new("Invalid doc"))?;
        
        let canvas = doc.get_element_by_id("canvas").ok_or_else(|| JsError::new("Invalid canvas"))?
                        .dyn_into::<HtmlCanvasElement>().map_err(|_| JsError::new("Invalid canvas"))?;

        let gctx = canvas.get_context("2d").map_err(|_| JsError::new("No 2d ctx"))?
                        .ok_or_else(|| JsError::new("No 2d ctx"))?
                        .dyn_into::<CanvasRenderingContext2d>().map_err(|_| JsError::new("Invalid 2d ctx"))?;

        let color_fg = color_fg.unwrap_or(String::from("white"));
        let color_bg = color_bg.unwrap_or(String::from("black"));
        
        Ok(Self {
            gctx,
            scale,
            color_fg,
            color_bg,
        })
    }
}

impl Graphics for JSGraphicsCtx {
    fn draw(&mut self, pixels: &PixelBits) -> Result<ProgramStatus, Box<dyn Error>> {
        self.gctx.set_fill_style_str(&self.color_bg);
        self.gctx.fill_rect(0.0, 0.0, WIDTH as f64 * self.scale as f64, HEIGHT as f64 * self.scale as f64);
        self.gctx.set_fill_style_str(&self.color_fg);

        for y in 0..HEIGHT as u8 {
            for x in 0..WIDTH as u8 {
                if pixels.get(x, y) != 0 {
                    self.gctx.fill_rect(
                        (x * self.scale) as f64,
                        (y * self.scale) as f64,
                        self.scale as f64,
                        self.scale as f64
                    );
                }
            }
        }

        // when putting thru wasm: result.map_err(|e| JsValue::from_str(&e.to_string()))
        Ok(ProgramStatus::Ok)
    }
}
