use lite_lib::component::Component;
use lite_lib::components::{button::Button, select::Select};
use lite_lib::listener::EventListener;
use lite_lib::utils::document;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let button = Button::new("My first button", document().body().unwrap(), || {
        log("An awsome mouse click")
    });
    button.render();

    EventListener::new(
        document()
            .get_element_by_id("button")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap(),
        "click",
        || log("An awsome click add by eventListener"),
    );

    let select = Select::new(
        "My first select",
        document().body().unwrap(),
        vec!["First", "Second", "Third"],
    );
    select.render();

    EventListener::new(
        document()
            .get_element_by_id("select")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap(),
        "change",
        || log("An awsome click add by eventListener in select with value"),
    );

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
