use js_sys::Function;
use lite_lib::component::{Children, Component, Renderer, Title};
use lite_lib::components::{
    button::Button, button::ConnectedButton, form::Form, form::FormElement, form::FormElementType, select::Select,
};
use lite_lib::layouts::modal::Modal;
use lite_lib::listener::EventListener;
use lite_lib::redux::{provider::Provider, store::Store, subscription::Subscription, provider::ConnectedComponent};
use lite_lib::utils::{dom::document, fetch::fetch_and_store_data, query_selector::SelectorType};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, RequestMode};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Default, Debug)]
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
    let provider = Arc::new(Mutex::new(Provider::new(store, document().body().unwrap())));
    let modal = Modal::new(document().body().unwrap());

    let select_driver = Select::new(None, vec!["Driver schedule"]);
    provider
        .lock()
        .unwrap()
        .add_child(Box::new(Arc::new(Mutex::new(select_driver))));

    let button_create_driver: Button<State> =
        Button::new("Create driver", append_driver_form(modal));
    let button = Arc::new(Mutex::new(button_create_driver));
    
    provider.lock().unwrap().connect_to_store(listener(Arc::clone(&button)));
    
    let moved_button = Arc::clone(&button);
    let callback = Box::new(move |button: &mut Button<State>, state: &State| {
        button.set_title(&state.data)
    });
    button.lock().unwrap().add_map_dispatch_to_props(Some(callback));
    
    let button_clone = Arc::clone(&button);
    provider.lock().unwrap().add_child(Box::new(button_clone));

    let provider_2 = provider.clone();

    let button_change_title = Button::new("Change title", change_title(provider_2));
    provider
        .lock()
        .unwrap()
        .add_child(Box::new(Arc::new(Mutex::new(button_change_title))));

    // Render the provider, which will render it children, so the entire components of the app
    provider.lock().unwrap().render();
    // // Add EventListener
    // EventListener::new(SelectorType::Id, "button", "click", on_button_click);
    // EventListener::new(SelectorType::Id, "select", "change", on_select_change);

    provider.lock().unwrap().dispatch_to_store(Action::Change("yolo".to_string()));

    Ok(())
}

fn append_driver_form(mut component: Modal) -> Function {
    let cb = Closure::wrap(Box::new(move || {
        let form_element_enter_lastname =
            FormElement::new("form_element", FormElementType::Input, Some("Lastname"));
        let form_element_enter_firstname =
            FormElement::new("form_element", FormElementType::Input, Some("Firstname"));
        let form = Form::new(
            "first_form",
            vec![form_element_enter_lastname, form_element_enter_firstname],
        );
        component.append_child(form);
    }) as Box<dyn FnMut()>);

    let res = cb.as_ref().to_owned().unchecked_into();
    Closure::forget(cb);
    res
}

fn change_title(provider: Arc<Mutex<Provider<State, Action>>>) -> Function {
    let cb = Closure::wrap(Box::new(move || {
        provider
            .lock()
            .unwrap()
            .dispatch_to_store(Action::Change("yolo".to_string()));
    }) as Box<dyn FnMut()>);

    let res = cb.as_ref().to_owned().unchecked_into();
    Closure::forget(cb);
    res
}

// fn un_truc_au_dessus(button: impl ConnectedComponent<State>) {
    
// }

fn listener(button: Arc<Mutex<Button<State>>>) -> Subscription<State> {
    Box::new(move |state| {
        button.lock().unwrap().dispatch(state)
    })
}

// fn map_dispatch_to_props_button(button: Arc<Mutex<Button<State>>>) {
//     let callback = Box::new(move |state: &State| {
//         button.lock().unwrap().set_title(&state.data)
//     });
//     button.lock().unwrap().add_map_dispatch_to_props(Some(callback));
// }

// fn listener(button_create_driver: &'static mut Button<State>) -> Subscription<State> {
//     Box::new(|state| {
//         button_create_driver.dispatch(state);
//     })
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
