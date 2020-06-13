use crate::utils::logger::logger;
use std::{collections::BTreeMap, sync::mpsc::Sender};

pub struct Storage {
    global_sender: BTreeMap<String, Sender<String>>,
    global_state: BTreeMap<String, String>,
}

impl Storage {
    pub fn new(
        first_sender: BTreeMap<String, Sender<String>>,
        first_state: BTreeMap<String, String>,
    ) -> Storage {
        Storage {
            global_sender: first_sender,
            global_state: first_state,
        }
    }

    pub fn add_btreemap(
        &mut self,
        sender: &mut BTreeMap<String, Sender<String>>,
        string: &mut BTreeMap<String, String>,
    ) {
        self.global_sender.append(sender);
        self.global_state.append(string);
    }

    pub fn update_state(&mut self, state_to_merge: BTreeMap<String, String>) {
        for (new_uid, new_state) in state_to_merge {
            for (old_uid, old_state) in &mut self.global_state {
                if old_uid == &new_uid {
                    *old_state = new_state;
                    break;
                }
            }
        }
        self.dispatch();
        logger(&self.global_state);
    }

    fn dispatch(&mut self) {
        for (state_uid, state_item) in &mut self.global_state {
            for (sender_uid, sender) in &mut self.global_sender {
                if state_uid == sender_uid {
                    sender.send(state_item.clone()).unwrap();
                }
            }
        }
    }
}
