use crate::utils::dom::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub enum SelectorType {
    Id,
}

pub fn query_selector(selector_type: SelectorType, selector: &str) -> HtmlElement {
    match selector_type {
        SelectorType::Id => document()
            .get_element_by_id(selector)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap(),
    }
}
