use crate::component::Component;
use crate::redux::{connect::Connect, provider::ConnectedComponent, map_dispatch_to_props::MapDispatchToProps};
use crate::utils::dom::*;
use js_sys::Function;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlElement};

pub struct Button {
    title: String,
    callback: Function,
}

impl Component for Button {
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

impl<State> Connect<State> for Button {
    fn connect(&self, state: &State) {
        &self.disptach(state);
    }
}

impl<State> MapDispatchToProps<State> for Button {
    fn disptach(&self, state: &State) {
    }
}

impl<State> ConnectedComponent<State> for Button {}

impl Button {
    pub fn new(title: &str, callback: Function) -> Button {
        let title = title.to_owned();

        Button { title, callback }
    }
}
