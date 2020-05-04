use lite_lib::component::{base::Base};
use lite_lib::components::{button::Button, select::Select};
use lite_lib::listener::EventListener;
use lite_lib::utils::{dom::document, fetch::fetch_and_log_data};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Create a Button element and add a on_click EventListener
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
        on_button_click,
    );
    // Create a Select element and add a on_change EventListener
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
        on_select_change,
    );

    Ok(())
}

fn on_select_change() {
    let mut req = RequestInit::new();
    req.method("GET");
    req.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init("http://127.0.0.1:7878/data", &req)
        .expect("Request could not be created");
    // Block until async shit is done
    spawn_local(fetch_and_log_data(request));
}

fn on_button_click() {
    let button = Button::new("My second button", document().body().unwrap(), || {
        log("An second awsome mouse click")
    });
    button.render();
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
