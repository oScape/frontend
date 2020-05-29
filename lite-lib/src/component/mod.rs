use crate::redux::provider::ConnectedComponent;
use web_sys::HtmlElement;

pub trait Renderer {
    fn render(&self);
}

pub trait Component {
    fn create_element(&self) -> HtmlElement;
}

pub trait Label {
    fn create_label(label: &String) -> HtmlElement;
}

pub trait Children {
    fn add_child(&mut self, child: Box<dyn ConnectedComponent>);
}
