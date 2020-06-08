use crate::text::Text;
use std::collections::BTreeMap;

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
        // I found the value of the uid key
        if let Some(uid) = new_btreemap.get("uid") {
            // for each btreemap in self.state
            for btreemap in self.state.iter_mut() {
                // if one have
                if btreemap.get("uid") == Some(&uid.clone()) {
                    if let Some(text) = btreemap.get_mut("text") {
                        let btm = new_btreemap.clone();
                        *text = String::from(btm.get("text").unwrap());
                        Storage::dispatch(btm)
                    }
                }
            }
        }
    }

    fn dispatch(btreemap: BTreeMap<String, String>) {
        Text::update_element(btreemap);
    }
}
