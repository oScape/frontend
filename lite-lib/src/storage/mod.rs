use std::collections::BTreeMap;
use crate::text::Text;

pub struct Storage {
    state: BTreeMap<String, String>
}

impl Storage {
    pub fn new(state: BTreeMap<String, String>) -> Storage {
        Storage {
            state
        }
    }

    pub fn get_element(&self, uid: String) -> Option<Text> {
        todo!()
        // match &self.state.get(uid.as_str()) {
        //     text => text,
        //     _ => None,
        // }
    }
}