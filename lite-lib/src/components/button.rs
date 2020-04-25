use crate::component::*;
use crate::utils::*;
use web_sys::{Element, HtmlElement};

pub struct Button {
    label: String,
    element: Element,
    parent: HtmlElement,
}

impl Component for Button {
    fn render(&self) {
        self.parent
            .append_child(&self.element)
            .expect("could not insert element");
    }

    fn create_element() -> Element {
        get_document()
            .create_element("div")
            .expect("could not create element")
    }
}

impl Button {
    pub fn new(label: &str, parent: HtmlElement) -> Button {
        let element = Button::create_element();
        let label = label.to_owned();

        Button {
            label,
            parent,
            element,
        }
    }
}
