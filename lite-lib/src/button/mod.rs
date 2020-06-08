use crate::utils::dom::document;
use crate::{
    storage::Storage,
    utils::query_selector::{query_selector, SelectorType},
};
use js_sys::Function;
use std::{collections::BTreeMap, sync::MutexGuard};
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    uid: String,
    text: String,
    on_click: Function,
}

impl Button {
    pub fn new(text: String, on_click: Function) -> Button {
        let uid = String::from("an_awsome_uid");
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

    pub fn update_element(btreemap: BTreeMap<String, String>) {
        let old_element = query_selector(SelectorType::Id, btreemap.get("uid").unwrap().as_str())
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        // old_element.set_onclick(Some(&on_click));
        old_element.set_inner_text(btreemap.get("text").unwrap().as_str());
    }

    // pub fn build_tree_map(&self) -> BTreeMap<String, String> {
    //     let mut btreemap = BTreeMap::new();
    //     btreemap.insert(String::from("uid"), String::from(&*self.uid));
    //     btreemap.insert(String::from("text"), String::from(&*self.text));

    //     btreemap
    // }
}
