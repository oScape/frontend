use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("should have a body on document");

    let p = document.create_element("p")?;
    p.set_inner_html("Hello from rust");
    
    body.append_child(&p)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}