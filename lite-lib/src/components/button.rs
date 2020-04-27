use crate::component::*;
use crate::utils::*;
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    label: String,
    parent: HtmlElement,
    event: fn(),
}

impl Component for Button {
    fn render(&self) {
        self.parent.append_child(&self.create_element()).unwrap();
    }

    fn create_element(&self) -> HtmlElement {
        let event = self.event.clone();
        let closure = Closure::wrap(Box::new(event) as Box<dyn Fn()>);
        let element = document()
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        element.set_onclick(Some(
            closure.as_ref().to_owned().unchecked_ref::<Function>(),
        ));
        closure.forget();
        element.set_inner_text(&self.label);
        element.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Button {
    pub fn new(label: &str, parent: HtmlElement, event: fn()) -> Button {
        let label = label.to_owned();

        Button {
            label,
            parent,
            event,
        }
    }
}
