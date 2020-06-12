use serde::{Serialize, Deserialize};
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
            global_state: first_state
        }
    }

    pub fn add_btreemap(&mut self, btreemap: &mut BTreeMap<String, String>) {
        self.global_state.append(btreemap);
    }

    pub fn update_state(&mut self, state_to_merge: BTreeMap<String, String>) {
        for (new_uid, new_state) in state_to_merge {
            for (old_uid, old_state) in &mut self.global_state {
                if old_uid == &new_uid {
                    *old_state = new_state;
                    break
                }
            }
        }
        Storage::dispatch(&self.global_state);
        // logger(&self.global_state);
    }

    fn dispatch(atomic_state: &BTreeMap<String, String>) {
        for (uid, item) in atomic_state {
            let text: Text = serde_json::from_str(item).unwrap();
            Text::update_element(text);
        }
    }
}
