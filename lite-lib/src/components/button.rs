use crate::component::Component;
use crate::store::{connect::Connect, provider::ConnectedComponent};
use crate::utils::dom::*;
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    title: String,
    callback: fn(),
}

impl Component for Button {
    fn create_element(&self) -> HtmlElement {
        let callback = self.callback.clone();
        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);
        let button = document()
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        button.set_id("button");
        button.set_onclick(Some(
            closure.as_ref().to_owned().unchecked_ref::<Function>(),
        ));
        closure.forget();
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
    pub fn new(title: &str, callback: fn()) -> Button {
        let title = title.to_owned();

        Button { title, callback }
    }
}
