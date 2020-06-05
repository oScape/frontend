use wasm_bindgen::prelude::*;
use lite_lib::text::Text;
use lite_lib::storage::Storage;
use web_sys::window;
use std::collections::BTreeMap;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let text_element = Text::new(String::from("text"));
    let mut btreemap = BTreeMap::new();
    btreemap.insert(String::from("div"), text_element);

    let storage = Storage::new(btreemap);
    let text_element = text_element.render_element();


    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&text_element)
        .unwrap();

    let text_element = Text::new(String::from("text"));
    text_element.update_element(String::from("div"));

    Ok(())
}

// use js_sys::Function;
// use lite_lib::component::{Children, Component, Renderer};
// use lite_lib::components::{
//     button::Button, form::Form, form::FormElement, form::FormElementType, select::Select,
// };
// use lite_lib::layouts::modal::Modal;
// use lite_lib::listener::EventListener;
// use lite_lib::redux::{provider::Provider, store::Store, subscription::Subscription};
// use lite_lib::utils::{dom::document, fetch::fetch_and_store_data, query_selector::SelectorType};
// use std::sync::{Arc, Mutex};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::{closure::Closure, JsCast};
// use wasm_bindgen_futures::spawn_local;
// use web_sys::{Request, RequestInit, RequestMode};

// #[derive(Default)]
// struct State {
//     data: String,
// }

// enum Action {
//     Change(String),
// }

// fn data_reducer(_state: &State, action: &Action) -> State {
//     match action {
//         Action::Change(data) => State { data: data.clone() },
//     }
// }

// #[wasm_bindgen(start)]
// pub fn run() -> Result<(), JsValue> {
//     let store = Store::new(data_reducer, State::default());
//     let provider = Arc::new(Mutex::new(Provider::new(store, document().body().unwrap())));
//     let modal = Modal::new(document().body().unwrap());

//     let listener: Subscription<State> = |state: &State| {
//         log(&format!("Counter changed! New value: {}", state.data));
//     };
//     provider.lock().unwrap().connect_to_store(listener);

//     let select_driver = Select::new(None, vec!["Driver schedule"]);
//     provider.lock().unwrap().add_child(Box::new(select_driver));

//     let button_create_driver = Button::new("Create driver", append_driver_form(modal));
//     provider
//         .lock()
//         .unwrap()
//         .add_child(Box::new(button_create_driver));

//     let provider_2 = provider.clone();

//     let button_change_title = Button::new("Change title", change_title(provider_2));
//     provider
//         .lock()
//         .unwrap()
//         .add_child(Box::new(button_change_title));

//     // Render the provider, which will render it children, so the entire components of the app
//     provider.lock().unwrap().render();
//     // // Add EventListener
//     // EventListener::new(SelectorType::Id, "button", "click", on_button_click);
//     // EventListener::new(SelectorType::Id, "select", "change", on_select_change);

//     // provider.dispatch_to_store(Action::Change("yolo".to_string()));

//     Ok(())
// }

// fn append_driver_form(mut component: Modal) -> Function {
//     let cb = Closure::wrap(Box::new(move || {
//         let form_element_enter_lastname =
//             FormElement::new("form_element", FormElementType::Input, Some("Lastname"));
//         let form_element_enter_firstname =
//             FormElement::new("form_element", FormElementType::Input, Some("Firstname"));
//         let form = Form::new(
//             "first_form",
//             vec![form_element_enter_lastname, form_element_enter_firstname],
//         );
//         component.append_child(form);
//     }) as Box<dyn FnMut()>);

//     let res = cb.as_ref().to_owned().unchecked_into();
//     Closure::forget(cb);
//     res
// }

// fn change_title(provider: Arc<Mutex<Provider<State, Action>>>) -> Function {
//     let cb = Closure::wrap(Box::new(move || {
//         provider
//             .lock()
//             .unwrap()
//             .dispatch_to_store(Action::Change("yolo".to_string()));
//     }) as Box<dyn FnMut()>);

//     let res = cb.as_ref().to_owned().unchecked_into();
//     Closure::forget(cb);
//     res
// }

// fn on_select_change() {
//     let mut req = RequestInit::new();
//     req.method("GET");
//     req.mode(RequestMode::Cors);
//     let request = Request::new_with_str_and_init("http://127.0.0.1:7878/data", &req)
//         .expect("Request could not be created");
//     // Block until async shit is done
//     spawn_local(fetch_and_store_data(request));
// }

// fn on_button_click() {
//     let button = Button::new(
//         "My second button",
//         || log("An second awsome mouse click"),
//         "UnknowInstance".to_string(),
//     );
//     button.render();
// }
