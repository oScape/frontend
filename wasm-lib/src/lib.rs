use js_sys::Function;
use lite_lib::{
    button::Button,
    storage::{Storage},
    text::Text,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let text_element = Text::new(String::from("text_uid_1"), String::from("text_1"));
    text_element.render_element();
    let storage = Arc::new(Mutex::new(Storage::new(text_element.build_tree_map())));

    let text_element_2 = Text::new(String::from("text_uid_2"), String::from("text_2"));
    text_element_2.render_element();
    storage
        .lock()
        .unwrap()
        .add_btreemap(&mut text_element_2.build_tree_map());

    let text_element_3 = Text::new(String::from("text_uid_3"), String::from("text_3"));
    text_element_3.render_element();
    storage
        .lock()
        .unwrap()
        .add_btreemap(&mut text_element_3.build_tree_map());

    let button_element = Button::new(
        String::from("button_uid"),
        String::from("button"),
    );
    button_element.render_element();
    button_element.add_on_click(
        on_click_action(storage.clone()));
    storage
        .lock()
        .unwrap()
        .add_btreemap(&mut button_element.build_tree_map());

    Ok(())
}

fn on_click_action(
    storage: Arc<Mutex<Storage>>
) -> Function {
    let new_button = Button::new(
        String::from("button_uid"),
        String::from("new_button"),
    );
    let new_text = Text::new(
        String::from("text_uid_1"),
        String::from("new_text"),
    );
    let cb = Closure::wrap(Box::new(move || {
        let mut btreemap = BTreeMap::new();
        btreemap.insert(String::from("button_uid"), serde_json::to_string(&new_button).unwrap());
        btreemap.insert(String::from("text_uid_1"), serde_json::to_string(&new_text).unwrap());

        storage.lock().unwrap().update_state(btreemap);
    }) as Box<dyn FnMut()>);

    let res = cb.as_ref().to_owned().unchecked_into();
    Closure::forget(cb);
    res
}
