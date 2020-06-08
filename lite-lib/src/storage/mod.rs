use std::collections::{BTreeSet, BTreeMap};
use std::mem;

#[derive(Default, Clone)]
pub struct Storage {
    state: Vec<BTreeMap<String, String>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage::default()
    }

    pub fn add_btreemap(&mut self, btreemap: BTreeMap<String, String>) {
        self.state.push(btreemap);
    }

    pub fn get_element(&self, uid: String) -> Option<BTreeMap<String, String>> {
        let mut result = None;
        for btreemap in self.state.iter() {
            if let Some(_) = btreemap.get(uid.as_str()) {
                result = Some(btreemap.clone())
            }
        }

        result
    }

    pub fn update_element(&mut self, new_btreemap: BTreeMap<String, String>) {
        if let Some(uid) = new_btreemap.get("uid") {
            for btreemap in self.state.iter_mut() {
                if let Some(test) = btreemap.get_mut(uid) {
                    mem::replace(&mut new_btreemap.clone(), btreemap)
                }
            }
        }
    }
}
