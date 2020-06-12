use std::collections::BTreeMap;
use web_sys::console;

pub fn logger<DTO>(new_state: &BTreeMap<String, DTO>) {
    console::group_1(&format!("action").into());
    console::group_collapsed_1(&format!("old state").into());
    // for (uid, item) in old_state {
    //     console::group_collapsed_1(&format!("{} uid: {}", item.element_type, uid).into());
    //     console::log_1(&format!("Text: {}", item.text).into());
    //     console::group_end();
    // }
    // console::group_end();
    // console::group_collapsed_1(&format!("new state").into());
    // for (uid, item) in new_state {
    //     console::group_collapsed_1(&format!("{} uid: {}", item.element_type, uid).into());
    //     console::log_1(&format!("Text: {}", item.text).into());
    //     console::group_end();
    // }
    console::group_end();
    console::group_end();
}
