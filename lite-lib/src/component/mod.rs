use crate::redux::provider::ConnectedComponent;
use web_sys::HtmlElement;
use std::sync::{Mutex, Arc};

pub trait Renderer {
    fn render(&self);
}

pub trait Component {
    fn create_element(&self) -> HtmlElement;
}

pub trait Title {
    fn sset_title(&mut self, title: &String);
}

pub trait Label {
    fn create_label(label: &String) -> HtmlElement;
}

pub trait Children<State> {
    fn add_child(&mut self, child: Box<Arc<Mutex<dyn ConnectedComponent<State>>>>);

    fn dispatch(&self, state: &State) {}
}
