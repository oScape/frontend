use crate::utils::*;
use web_sys::{HtmlElement, Element};

pub struct Button {
    label: String,
    parent: HtmlElement,
    element: Element
}

impl Button {
    pub fn new(label: &str, parent: HtmlElement) -> Button {
        let element = Button::create_element(&label);
        let label = label.to_owned();

        Button { label, parent, element }
    }

    pub fn render(&self) {
        self.parent.append_child(&self.element).expect("could not insert element");
    }

    fn create_element(label: &str) -> Element {
        let element = get_document().create_element("div").expect("could not create element");
        element.set_inner_html(&label);
        element
    }
}
