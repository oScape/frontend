use web_sys::HtmlElement;

pub trait Label {
    fn create_label(&self) -> HtmlElement;
}
