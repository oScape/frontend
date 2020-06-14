use crate::utils::dom::document;
use crate::utils::query_selector::{query_selector, SelectorType};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::mpsc::{channel, Sender, Receiver},
};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, HtmlDivElement, HtmlElement};
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize)]
pub struct Text {
    type_element: String,
    uid: String,
    text: String,
}

impl Text {
    pub fn new(uid: String, text: String) -> Text {
        let type_element = String::from("text");
        Text {
            type_element,
            text,
            uid,
        }
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

    pub fn get_type_element(&self) -> &String {
        &self.type_element
    }

    pub fn update_element(item: Text) {
        let old_element = query_selector(SelectorType::Id, item.uid.as_str())
            .dyn_into::<HtmlDivElement>()
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
        spawn_local(Text::spawner(receiver));

        // move || while let Ok(new_text) = receiver.recv() {
        //     my_paragraph.set_text(new_text);
        // }
        // let closure = Closure::wrap(Box::new(move || while let Ok(received_data) = receiver.recv() {
        //     let received_data: String = received_data;
        //     Text::update_element(serde_json::from_str(received_data.as_str()).unwrap());
        // }) as Box<dyn FnMut()>);
        // let closure = Closure::wrap(Box::new(move || while let Ok(received_data) {
        //     Text::update_element(serde_json::from_str(received_data.as_str()).unwrap());
        // }) as Box<dyn FnMut()>);
        // Closure::forget(closure);

        let mut btreemap = BTreeMap::new();
        btreemap.insert(String::from(&*self.uid), sender);

        btreemap
    }

    pub async fn spawner(receiver: Receiver<String>) {
        move || while let Ok(received_data) = receiver.recv() {
            let received_data: String = received_data;
            Text::update_element(serde_json::from_str(received_data.as_str()).unwrap());
        };
    }
}
