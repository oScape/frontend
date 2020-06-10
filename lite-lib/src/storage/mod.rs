use crate::{button::Button, text::Text};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ItemDTO {
    pub element_type: String,
    pub text: String,
}

#[derive(Default)]
pub struct Storage {
    global_state: Vec<BTreeMap<String, ItemDTO>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_btreemap(&mut self, btreemap: BTreeMap<String, ItemDTO>) {
        self.global_state.push(btreemap);
    }

    pub fn get_element(&self, uid: String) -> Option<BTreeMap<String, ItemDTO>> {
        let mut result = None;
        for btreemap in self.global_state.iter() {
            if let Some(_) = btreemap.get(uid.as_str()) {
                result = Some(btreemap.clone())
            }
        }

        result
    }

    pub fn update_state(&mut self, mut state_to_merge: BTreeMap<String, ItemDTO>) {
        for (new_uid, _) in state_to_merge.clone().iter() {
            for atomic_state in self.global_state.iter_mut() {
                for (old_uid, _) in atomic_state.clone().iter() {
                    if old_uid == new_uid {
                        atomic_state.clear();
                        atomic_state.append(&mut state_to_merge);
                        Storage::dispatch(&atomic_state);
                    }
                }
            }
        }
    }

    fn dispatch(atomic_state: &BTreeMap<String, ItemDTO>) {
        for (uid, item) in atomic_state.iter() {
            match item.element_type.as_str() {
                "text" => Text::update_element(uid.clone(), item.clone()),
                "button" => Button::update_element(uid.clone(), item.clone()),
                _ => (),
            }
        }
    }
}
