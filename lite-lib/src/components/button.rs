use crate::component::Component;
use web_sys::HtmlElement;

pub struct Button {
    label: String,
    pub component: Component,
}

impl Button {
    pub fn new(label: &str, parent: HtmlElement) -> Button {
        let label = label.to_owned();
        let component = Component::new(parent);

        Button { label, component }
    }
}
