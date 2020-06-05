use std::collections::BTreeMap;
use crate::text::Text;

pub struct Storage {
    state: BTreeMap<String, Text>
}

impl Storage {
    pub fn new(state: BTreeMap<String, Text>) -> Storage {
        Storage {
            state
        }
    }

    pub fn get_element(&self, uid: String) -> Option<Text> {
        match &self.state.get(uid.as_str()) {
            text => text,
            _ => None,
        }
    }
}