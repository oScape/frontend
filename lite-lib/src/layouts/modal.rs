use crate::component::{Component, Renderer};
use crate::utils::dom::*;
use crate::utils::query_selector::{query_selector, SelectorType};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

pub struct Modal {
    parent: HtmlElement,
}

impl Renderer for Modal {
    fn render(&self) {
        let element: HtmlElement = self.create_element();
        self.parent.append_child(&element).unwrap();
    }
}

impl Component for Modal {
    fn create_element(&self) -> HtmlElement {
        let modal = document()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        modal.set_id("modal");

        modal.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Modal {
    pub fn new(parent: HtmlElement) -> Self {
        Self { parent }
    }

    pub fn append_child(&mut self, c: impl Component) {
        self.render();
        let target_element = query_selector(SelectorType::Id, "modal");
        target_element.append_child(&c.create_element()).unwrap();
    }
}
