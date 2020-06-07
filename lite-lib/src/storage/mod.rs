use crate::text::Text;
use std::collections::BTreeMap;

pub struct Storage {
    state: BTreeMap<String, String>,
}

impl Storage {
    pub fn new(state: BTreeMap<String, String>) -> Storage {
        Storage { state }
    }

    pub fn get_element(&self, uid: String) -> Option<String> {
        match self.state.get(uid.as_str()) {
            Some(text) => Some(String::from(&*text)),
            None => None,
        }
    }

    pub fn update_element(&mut self, uid: String, data: String) {
        if let Some(text) = self.state.get_mut(uid.as_str()) {
            *text = String::from(&*data);
        };
        Text::update_element(String::from(uid), data);
    }
}
