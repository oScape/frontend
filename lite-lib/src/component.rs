use crate::utils::*;
use web_sys::{Element, HtmlElement};

pub struct Component {
    parent: HtmlElement,
    element: Element,
}

impl Component {
    pub fn new(parent: HtmlElement) -> Component {
        let element = Component::create_element();

        Component { parent, element }
    }

    pub fn render(&self) {
        self.parent
            .append_child(&self.element)
            .expect("could not insert element");
    }

    fn create_element() -> Element {
        let element = get_document()
            .create_element("div")
            .expect("could not create element");
        element
    }
}
