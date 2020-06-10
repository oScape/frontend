use js_sys::Function;
use lite_lib::{
    button::Button,
    storage::{ItemDTO, Storage},
    text::Text,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
use wasm_bindgen::{prelude::*, JsCast};


#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let text_element = Text::new(String::from("text"));
    text_element.render_element();

    let storage = Arc::new(Mutex::new(Storage::new()));
    storage
        .lock()
        .unwrap()
        .add_btreemap(text_element.build_tree_map());

    let new_item = ItemDTO {
        element_type: String::from("button"),
        text: String::from("new_item"),
    };

    let new_item_2 = ItemDTO {
        element_type: String::from("text"),
        text: String::from("new_item_2"),
    };

    let button_element = Button::new(
        String::from("button"),
        on_click_action(storage.clone(), new_item, new_item_2),
    );
    button_element.render_element();

    storage
        .lock()
        .unwrap()
        .add_btreemap(button_element.build_tree_map());

    Ok(())
}

fn on_click_action(storage: Arc<Mutex<Storage>>, new_item: ItemDTO, new_item_2: ItemDTO ) -> Function {
    let cb = Closure::wrap(Box::new(move || {
        let mut btreemap = BTreeMap::new();
        btreemap.insert(String::from("an_uid"), new_item.clone());
        btreemap.insert(String::from("an_awsome_uid"), new_item_2.clone());

        storage.lock().unwrap().update_state(btreemap);
    }) as Box<dyn FnMut()>);

    let res = cb.as_ref().to_owned().unchecked_into();
    Closure::forget(cb);
    res
}
