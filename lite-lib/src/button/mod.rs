use crate::utils::dom::document;
use crate::utils::query_selector::{query_selector, SelectorType};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::mpsc::{channel, Sender},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlButtonElement, HtmlElement};

#[derive(Serialize, Deserialize)]
pub struct Button {
    type_element: String,
    uid: String,
    text: String,
}

impl Button {
    pub fn new(uid: String, text: String) -> Button {
        let type_element = String::from("button");
        Button {
            type_element,
            text,
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
        element.set_inner_text(self.text.as_str());

        let element = element.dyn_into::<HtmlElement>().unwrap();
        document().body().unwrap().append_child(&element).unwrap();

        &self
    }

    pub fn get_type_element(&self) -> &String {
        &self.type_element
    }

    pub fn add_on_click(&self, on_click: Function) {
        let element = query_selector(SelectorType::Id, &self.uid.as_str())
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        element.set_onclick(Some(&on_click));
    }

    pub fn update_element(item: Button) {
        let old_element = query_selector(SelectorType::Id, item.uid.as_str())
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        old_element.set_inner_text(item.text.as_str());
    }

    pub fn build_tree_map(&self) -> BTreeMap<String, String> {
        let mut btreemap = BTreeMap::new();
        btreemap.insert(
            String::from(&*self.uid),
            serde_json::to_string(&self).unwrap(),
        );

        btreemap
    }

    pub fn build_tree_sender(&self) -> BTreeMap<String, Sender<String>> {
        let (sender, receiver) = channel();
        let closure = Closure::wrap(Box::new(move || {
            let received_data: String = receiver.recv().unwrap();
            Button::update_element(serde_json::from_str(received_data.as_str()).unwrap());
        }) as Box<dyn FnMut()>);
        Closure::forget(closure);

        let mut btreemap = BTreeMap::new();
        btreemap.insert(String::from(&*self.uid), sender);

        btreemap
    }
}
