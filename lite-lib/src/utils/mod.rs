use web_sys::{Document, HtmlElement};

pub fn get_document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

pub fn get_body() -> HtmlElement {
    get_document().body().expect("should have a body on document")
}