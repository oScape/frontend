use web_sys::Element;

pub trait Component {
    fn render(&self);
    fn create_element() -> Element;
}
