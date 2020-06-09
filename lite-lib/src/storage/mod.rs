use crate::{button::Button, text::Text};
use js_sys::Function;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ItemDTO {
    pub element_type: String,
    pub text: String,
    pub on_click: Option<Function>,
}

#[derive(Default)]
pub struct Storage {
    state: Vec<BTreeMap<String, ItemDTO>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_btreemap(&mut self, btreemap: BTreeMap<String, ItemDTO>) {
        self.state.push(btreemap);
    }

    pub fn get_element(&self, uid: String) -> Option<BTreeMap<String, ItemDTO>> {
        let mut result = None;
        for btreemap in self.state.iter() {
            if let Some(_) = btreemap.get(uid.as_str()) {
                result = Some(btreemap.clone())
            }
        }

        result
    }

    pub fn update_state(&mut self, mut new_btreemap: BTreeMap<String, ItemDTO>) {
        for (new_uid, _) in new_btreemap.clone().iter() {
            for old_btreemap in self.state.iter_mut() {
                for (old_uid, _) in old_btreemap.clone().iter() {
                    if old_uid == new_uid {
                        old_btreemap.clear();
                        old_btreemap.append(&mut new_btreemap);
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
