use crate::utils::dom::document;
use crate::{
    storage::ItemDTO,
    utils::query_selector::{query_selector, SelectorType},
};
use js_sys::Function;
use std::collections::BTreeMap;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    uid: String,
    text: String,
    on_click: Function,
}

impl Button {
    pub fn new(uid: String, text: String, on_click: Function) -> Button {
        Button {
            text,
            on_click,
            uid,
        }
    }

    pub fn render_element(&self) -> &Button {
        let element = document()
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        element.set_id(self.uid.as_str());
        element.set_onclick(Some(&self.on_click));
        element.set_inner_text(self.text.as_str());

        let element = element.dyn_into::<HtmlElement>().unwrap();
        document().body().unwrap().append_child(&element).unwrap();

        &self
    }

    pub fn update_element(uid: String, item: ItemDTO) {
        let old_element = query_selector(SelectorType::Id, uid.as_str())
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        old_element.set_inner_text(item.text.as_str());
    }

    pub fn build_tree_map(&self) -> BTreeMap<String, ItemDTO> {
        let mut btreemap = BTreeMap::new();
        let new_item = ItemDTO {
            element_type: String::from("button"),
            text: String::from(&*self.text),
        };
        btreemap.insert(String::from(&*self.uid), new_item);

        btreemap
    }
}
