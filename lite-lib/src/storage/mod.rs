use std::collections::BTreeMap;

pub struct Storage {
    state: BTreeMap<String, String>
}

impl Storage {
    pub fn new(state: BTreeMap<String, String>) -> Storage {
        Storage {
            state
        }
    }

    pub fn get_element(&self, uid: String) -> Option<String> {
        match self.state.get(uid.as_str()) {
            Some(text) => Some(String::from(&*text)),
            None => None,
        }
    }
}