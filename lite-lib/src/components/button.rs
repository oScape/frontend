use crate::component::*;
use crate::utils::*;
use web_sys::{Element, HtmlElement};

pub struct Button {
    label: String,
    parent: HtmlElement,
}

impl Component for Button {
    fn render(&self) {
        self.parent
            .append_child(&self.create_element())
            .expect("could not insert element");
    }

    fn create_element(&self) -> Element {
        let element = get_document()
            .create_element("div")
            .expect("could not create element");
        element.set_inner_html(&self.label);
        element
    }
}

impl Button {
    pub fn new(label: &str, parent: HtmlElement) -> Button {
        let label = label.to_owned();

        Button { label, parent }
    }
}
