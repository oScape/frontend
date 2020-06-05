use web_sys::{window, HtmlElement, HtmlDivElement};
use wasm_bindgen::JsCast;

pub struct Text {
    text: String
}

impl Text {
    pub fn new(text: String) -> Text {
        Text {
            text
        }
    }

    pub fn render_element(self) -> HtmlElement {
        let element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        element.set_id("div");

        element.set_inner_text(self.text.as_str());

        element.dyn_into::<HtmlElement>().unwrap()
    }

    pub fn update_element(self, uid: String) {
        let old_element = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(uid.as_str())
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        old_element.set_inner_text("self.text.as_str()");
    }
}