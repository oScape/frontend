use lite_lib::component::{Children, Renderer};
use lite_lib::components::{
    button::Button, form::Form, form::FormElement, form::FormElementType, select::Select,
};
use lite_lib::listener::EventListener;
use lite_lib::store::{provider::Provider, store::Store, subscription::Subscription};
use lite_lib::utils::{dom::document, fetch::fetch_and_store_data, query_selector::SelectorType};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, RequestMode};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Default)]
struct State {
    data: String,
}

enum Action {
    Change(String),
}

fn data_reducer(_state: &State, action: &Action) -> State {
    match action {
        Action::Change(data) => State { data: data.clone() },
    }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let store = Store::new(data_reducer, State::default());
    let mut provider = Provider::new(store, document().body().unwrap());
    // let listener: Subscription<State> = |state: &State| {
    //     log(&format!("Counter changed! New value: {}", state.data));
    // };
    // provider.connect_to_store(listener);
    let select_driver = Select::new(None, vec!["Driver schedule"]);
    provider.add_child(Box::new(select_driver));

    let button_create_driver = Button::new("Create driver", || log("I want to create a driver!"));
    provider.add_child(Box::new(button_create_driver));

    let form_element_enter_lastname = FormElement::new(
        "form_element",
        FormElementType::Input,
        Some("Lastname"),
    );
    let form_element_enter_firstname = FormElement::new(
        "form_element",
        FormElementType::Input,
        Some("Firstname"),
    );
    let form = Form::new("first_form", vec![form_element_enter_lastname, form_element_enter_firstname]);
    provider.add_child(Box::new(form));
    // Render the provider, which will render it children, so the entire components of the app
    provider.render();
    // // Add EventListener
    // EventListener::new(SelectorType::Id, "button", "click", on_button_click);
    // EventListener::new(SelectorType::Id, "select", "change", on_select_change);

    // provider.dispatch_to_store(Action::Change("yolo".to_string()));

    Ok(())
}

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
