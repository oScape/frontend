use crate::component::base::Base;
use crate::utils::dom::*;
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    title: String,
    parent: HtmlElement,
    callback: fn(),
}

impl Base for Button {
    fn render(&self) {
        self.parent.append_child(&self.create_element()).unwrap();
    }

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

impl Button {
    pub fn new(title: &str, parent: HtmlElement, callback: fn()) -> Button {
        let title = title.to_owned();

        Button {
            title,
            parent,
            callback,
        }
    }
}
