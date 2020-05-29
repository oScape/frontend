use super::dom::*;
use crate::redux::{store::Store, subscription::Subscription};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub async fn fetch_and_store_data(request: Request) {
    // Create the store
    let mut store = Store::new(data_reducer, State::default());

    let listener: Subscription<State> = |state: &State| {
        log(&format!("Counter changed! New value: {}", state.data));
    };

    store.subscribe(listener);

    // Fetch data from the server
    let response = JsFuture::from(window().fetch_with_request(&request))
        .await
        .expect("Could not unwrap response");

    // `response` is a `Response` object.
    assert!(response.is_instance_of::<Response>());
    let resp: Response = response.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let ouput = JsFuture::from(resp.text().unwrap())
        .await
        .unwrap()
        .as_string()
        .unwrap();

    store.dispatch(Action::Change(ouput));
}
