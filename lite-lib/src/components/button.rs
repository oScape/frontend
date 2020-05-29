use crate::component::Component;
use crate::redux::{connect::Connect, provider::ConnectedComponent};
use crate::utils::dom::*;
use js_sys::Function;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    title: String,
    callback: Function,
}

impl Component for Button {
    fn create_element(&self) -> HtmlElement {
        let button = document()
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        button.set_id("button");

        button.set_onclick(Some(&self.callback));
        button.set_inner_text(&self.title);

        button.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Connect for Button {
    fn connect(&mut self, data: String) {
        self.title = data;
    }
}

impl ConnectedComponent for Button {}

impl Button {
    pub fn new(title: &str, callback: Function) -> Button {
        let title = title.to_owned();

        Button { title, callback }
    }
}
