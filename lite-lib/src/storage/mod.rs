use crate::utils::logger::logger;
use crate::{button::Button, text::Text};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Storage {
    global_state: BTreeMap<String, String>,
}

impl Storage {
    pub fn new(first_state: BTreeMap<String, String>) -> Storage {
        Storage {
            global_state: first_state,
        }
    }

    pub fn add_btreemap(&mut self, btreemap: &mut BTreeMap<String, String>) {
        self.global_state.append(btreemap);
    }

    pub fn update_state(&mut self, state_to_merge: BTreeMap<String, String>) {
        logger(String::from("previous state"), &self.global_state);
        for (new_uid, new_state) in state_to_merge {
            for (old_uid, old_state) in &mut self.global_state {
                if old_uid == &new_uid {
                    *old_state = new_state;
                    break;
                }
            }
        }
        Storage::dispatch(&self.global_state);
        logger(String::from("next state"), &self.global_state);
    }

    fn dispatch(global_state: &BTreeMap<String, String>) {
        for (_, item) in global_state {
            let button: Button = serde_json::from_str(item).unwrap();
            if button.get_type_element().as_str() == "button" {
                Button::update_element(button);
            }
            let text: Text = serde_json::from_str(item).unwrap();
            if text.get_type_element().as_str() == "text" {
                Text::update_element(text);
            }
        }
    }
}
