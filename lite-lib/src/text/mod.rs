use crate::utils::dom::document;
use crate::{
    storage::ItemDTO,
    utils::query_selector::{query_selector, SelectorType},
};
use std::collections::BTreeMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

pub struct Text {
    uid: String,
    text: String,
}

impl Text {
    pub fn new(text: String) -> Text {
        let uid = String::from("an_awsome_uid");
        Text { text, uid }
    }

    pub fn render_element(&self) -> &Text {
        let element = document()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        element.set_id(self.uid.as_str());
        element.set_inner_text(self.text.as_str());

        let element = element.dyn_into::<HtmlElement>().unwrap();
        document().body().unwrap().append_child(&element).unwrap();

        &self
    }

    pub fn update_element(uid: String, item: ItemDTO) {
        let old_element = query_selector(SelectorType::Id, uid.as_str())
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        old_element.set_inner_text(item.text.as_str());
    }

    pub fn build_tree_map(&self) -> BTreeMap<String, ItemDTO> {
        let mut btreemap = BTreeMap::new();
        let new_item = ItemDTO {
            element_type: String::from("text"),
            text: String::from(&*self.text),
        };
        btreemap.insert(String::from(&*self.uid), new_item);

        btreemap
    }
}
