use crate::component::{Component, Title};
use crate::redux::{connect::Connect, provider::ConnectedComponent, subscription::Subscription};
use crate::utils::dom::*;
use wasm_bindgen::prelude::*;
use js_sys::Function;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement};

pub trait ConnectedButton<State>: Title + ConnectedComponent<State> {
    fn set_title(&mut self, title: &String) {
        self.sset_title(title);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Button<State> {
    title: String,
    callback: Function,
    map_dispatch_to_props: Option<Box<dyn Fn(&mut Button<State>, &State)>>
}

impl<State> Title for Button<State> {
    fn sset_title(&mut self, title: &String) {
        self.title = title.to_owned();
        document().body().unwrap().append_child(&self.create_element()).unwrap();
    }
}

impl<State> Component for Button<State> {
    fn create_element(&self) -> HtmlElement {
        let button = document()
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        button.set_id("button");

        button.set_onclick(Some(&self.callback));
        button.set_inner_text(&self.title);

        button.dyn_into::<HtmlElement>().unwrap()
    }
}

impl<State> ConnectedButton<State> for Button<State> {}

impl<State> Connect<State> for Button<State> {
    fn connect(&mut self, data: String) {
        self.title = data;
    }
}

impl<State> ConnectedComponent<State> for Button<State> {
    fn dispatch(&mut self, state: &State) {
        if let Some(ds) = &self.map_dispatch_to_props {
            ds(&mut self, state);
        }
    }
}

impl<State> Button<State> {
    pub fn new(title: &str, callback: Function) -> Button<State> {
        let title = title.to_owned();

        Button {
            title,
            callback,
            map_dispatch_to_props: None
        }
    }

    pub fn add_map_dispatch_to_props(&mut self, map_dispatch_to_props: Option<Box<dyn Fn(&mut Button<State>, &State)>>) {
        self.map_dispatch_to_props = map_dispatch_to_props;
    }
}
