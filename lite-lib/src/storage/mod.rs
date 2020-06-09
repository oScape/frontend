use crate::{button::Button, text::Text};
use js_sys::Function;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Item {
    pub element_type: String,
    pub text: String,
    pub on_click: Option<Function>,
}

#[derive(Default)]
pub struct Storage {
    state: Vec<BTreeMap<String, Item>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_btreemap(&mut self, btreemap: BTreeMap<String, Item>) {
        self.state.push(btreemap);
    }

    pub fn get_element(&self, uid: String) -> Option<BTreeMap<String, Item>> {
        let mut result = None;
        for btreemap in self.state.iter() {
            if let Some(_) = btreemap.get(uid.as_str()) {
                result = Some(btreemap.clone())
            }
        }

        result
    }

    pub fn update_state(&mut self, new_btreemap: BTreeMap<String, Item>) {
        for (new_uid, new_item) in new_btreemap.iter() {
            for old_btreemap in self.state.iter_mut() {
                for (old_uid, old_item) in old_btreemap.clone().iter() {
                    if old_uid == new_uid {
                        old_btreemap.clear();
                        old_btreemap.append(&mut new_btreemap.clone());
                    }
                }
            }
        }
        self.dispatch();
    }

    fn dispatch(&self) {
        for btreemap in self.state.iter() {
            for (uid, item) in btreemap.iter() {
                match item.element_type.as_str() {
                    "text" => Text::update_element(uid.clone(), item.clone()),
                    "button" => Button::update_element(uid.clone(), item.clone()),
                    _ => (),
                }
            }
        }
    }
}
