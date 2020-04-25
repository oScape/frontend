use lite_lib::components::button::*;
use lite_lib::utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let button = Button::new("My first button", utils::get_body());
    button.component.render();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
