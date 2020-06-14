use serde_json::{Map, Value};
use std::collections::BTreeMap;
use web_sys::console;

pub fn logger(state_type: String, state: &BTreeMap<String, String>) {
    console::group_collapsed_1(&format!("{}", state_type).into());
    for (_, item) in state {
        let v: Value = serde_json::from_str(item).unwrap();
        if v.is_object() {
            display_object(v.as_object().unwrap());
        } else if v.is_array() {
            display_array(v.as_array().unwrap());
        }
    }
    console::group_end();
}

fn display_object(object: &Map<String, Value>) {
    console::group_collapsed_1(&format!("Object").into());
    for (key, value) in object.iter() {
        if value.is_array() {
            display_array(value.as_array().unwrap());
        } else {
            display_primitive(key, value);
        }
    }
    console::group_end();
}

fn display_array(_: &Vec<Value>) {
    // TODO
}

fn display_primitive(key: &String, primitive: &Value) {
    console::log_1(&format!("{}: {}", key, primitive).into());
}
