use lite_lib::component::Component;
use lite_lib::components::{button::Button, select::Select};
use lite_lib::utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let button = Button::new("My first button", utils::document().body().unwrap(), || {
        log("Clicked")
    });
    button.render();

    let select = Select::new(
        "My first select",
        utils::document().body().unwrap(),
        vec!["First", "Second", "Third"],
    );
    select.render();

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
