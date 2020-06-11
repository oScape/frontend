use web_sys::console;
use std::collections::BTreeMap;
use crate::storage::ItemDTO;

pub fn logger(state_to_merge: BTreeMap<String, ItemDTO>) {
  console::group_collapsed_1(&format!("action").into());
  for (uid, item) in state_to_merge {
      console::group_collapsed_1(&format!("Item uid: {}", uid).into());
      console::log_1(&format!("Type: {}", item.element_type).into());
      console::log_1(&format!("Text: {}", item.text).into());
      console::group_end();
    }
    console::group_end();
}