use web_sys::HtmlElement;

pub trait Base {
    fn render(&self);

    fn create_element(&self) -> HtmlElement;
}

pub trait Label {
    fn create_label(&self) -> HtmlElement;
}
