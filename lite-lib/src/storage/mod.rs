use crate::utils::log::log;
use crate::{button::Button, text::Text};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ItemDTO {
    pub element_type: String,
    pub text: String,
}

#[derive(Default)]
pub struct Storage {
    global_state: BTreeMap<String, ItemDTO>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_btreemap(&mut self, btreemap: &mut BTreeMap<String, ItemDTO>) {
        self.global_state.append(btreemap);
    }

    pub fn update_state(&mut self, state_to_merge: BTreeMap<String, ItemDTO>) {
        for (new_uid, new_state) in &state_to_merge {
            for (old_uid, old_state) in &mut self.global_state {
                if old_uid == new_uid {
                    *old_state = new_state.clone();
                }
            }
        }
        Storage::dispatch(self.global_state.clone());
        Storage::logger(state_to_merge);
    }

    fn dispatch(atomic_state: BTreeMap<String, ItemDTO>) {
        for (uid, item) in atomic_state.iter() {
            match item.element_type.as_str() {
                "text" => Text::update_element(uid.clone(), item.clone()),
                "button" => Button::update_element(uid.clone(), item.clone()),
                _ => (),
            }
        }
    }

    fn logger(state_to_merge: BTreeMap<String, ItemDTO>) {
        for (uid, item) in state_to_merge {
            log(format!("UID: {}", uid).as_str());
            log("| Item:");
            log(format!("  | Type: {}", item.element_type).as_str());
            log(format!("  | Text: {}", item.text).as_str());
        }
    }
}
