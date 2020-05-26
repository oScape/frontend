use crate::store::provider::ConnectedComponent;
use web_sys::HtmlElement;

pub trait Renderer {
    fn render(&self);

    fn render_as_modal(&self);
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
