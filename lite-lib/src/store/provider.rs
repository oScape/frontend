use super::store::Store;
use crate::component::Base;
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};

// What could I use here to the representation of all the intance of elements
pub type UnknowInstance = String;

pub struct Provider<State, Action> {
    _store: Store<State, Action>,
    _parent: UnknowInstance,
    _children: Option<Vec<UnknowInstance>>,
}

impl<State, Action> Base for Provider<State, Action> {
    fn render(&self) {
        document()
            .body()
            .unwrap()
            .append_child(&self.create_element())
            .unwrap();
        // self.parent.append_child(&self.create_element()).unwrap();
    }

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

impl<State, Action> Provider<State, Action> {
    pub fn new(store: Store<State, Action>, parent: UnknowInstance) -> Provider<State, Action> {
        Provider {
            _store: store,
            _parent: parent,
            _children: None,
        }
    }
}
