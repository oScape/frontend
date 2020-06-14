use crate::utils::logger::logger;
use crate::{button::Button, text::Text};
use std::collections::{BTreeMap, HashMap};

pub type Tupplewear = (String, String);
#[derive(Default)]
pub struct Storage {
    global_state: BTreeMap<String, String>,
    global_convert: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_mappers(&mut self, btreemap: &mut BTreeMap<String, String>, tupplewear: Tupplewear) {
        self.global_state.append(btreemap);
        self.global_convert.insert(tupplewear.0, tupplewear.1);
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
        self.dispatch();
        logger(String::from("next state"), &self.global_state);
    }

    fn dispatch(&self) {
        for (uid, item) in &self.global_state {
            let convertor = self.global_convert.get(uid).unwrap();
            if convertor == "button" {
                let button: Button = serde_json::from_str(&item).unwrap();
                Button::update_element(button);
            } else if convertor == "text" {
                let text: Text = serde_json::from_str(&item).unwrap();
                Text::update_element(text);
            }
        }
    }
}
