use super::{connect::Connect, store::Store, subscription::Subscription};
use crate::component::{Children, Component, Renderer};
use crate::utils::dom::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

pub trait ConnectedComponent: Component + Connect {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Provider<State, Action> {
    _store: Store<State, Action>,
    parent: HtmlElement,
    children: Vec<Box<dyn ConnectedComponent>>,
}

impl<State, Action> Renderer for Provider<State, Action> {
    fn render(&self) {
        let element: HtmlElement = self.create_element();
        for child in &self.children {
            element.append_child(&child.create_element()).unwrap();
        }
        self.parent.append_child(&element).unwrap();
    }
}

impl<State, Action> Component for Provider<State, Action> {
    fn create_element(&self) -> HtmlElement {
        let provider = document()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        provider.set_id("provider");

        provider.dyn_into::<HtmlElement>().unwrap()
    }
}

impl<State, Action> Children for Provider<State, Action> {
    fn add_child(&mut self, child: Box<dyn ConnectedComponent>) {
        self.children.push(child);
    }
}

impl<State, Action> Provider<State, Action> {
    pub fn new(store: Store<State, Action>, parent: HtmlElement) -> Provider<State, Action> {
        Provider {
            _store: store,
            parent,
            children: Vec::new(),
        }
    }

    pub fn connect_to_store(&mut self, listener: Subscription<State>) {
        self._store.subscribe(listener);
    }

    pub fn dispatch_to_store(&mut self, action: Action) {
        self._store.dispatch(action);
    }
}
