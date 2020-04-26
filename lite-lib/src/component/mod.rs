use web_sys::HtmlElement;

pub trait Component {
    fn render(&self);

    fn create_element(&self) -> HtmlElement;
}
