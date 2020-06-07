use web_sys::{window, HtmlElement, HtmlDivElement};
use wasm_bindgen::JsCast;
use std::collections::BTreeMap;

pub struct Text {
    uid: String,
    text: String
}

impl Text {
    pub fn new(text: String) -> Text {
        let uid = String::from("an_awsome_uid");
        Text {
            text,
            uid
        }
    }

    pub fn render_element(&self) -> &Text {
        let element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        element.set_id(self.uid.as_str());

        element.set_inner_text(self.text.as_str());

        let element = element.dyn_into::<HtmlElement>().unwrap();

        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap();

        &self
    }

    pub fn update_element(uid: String) {
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

    pub fn build_tree_map(&self) -> BTreeMap<String, String> {
        let mut btreemap = BTreeMap::new();
        btreemap.insert(String::from("uid"),String::from(&*self.uid));
        btreemap.insert(String::from("text"),String::from(&*self.text));

        btreemap
    }
}