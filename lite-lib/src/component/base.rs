use web_sys::HtmlElement;

pub trait Base {
    fn render(&self);

    fn create_element(&self) -> HtmlElement;
}
