use crate::component::{Component, Label};
use crate::redux::{connect::Connect, provider::ConnectedComponent};
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlDivElement, HtmlElement, HtmlLabelElement, HtmlOptionElement, HtmlSelectElement,
};

pub struct Select {
    label: Option<String>,
    options: Vec<String>,
}

impl Component for Select {
    fn create_element(&self) -> HtmlElement {
        // Create the wrapper
        let wrapper = document()
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlDivElement>()
            .unwrap();
        wrapper.set_id("select-wrapper");

        let label: Option<HtmlElement> = match &self.label {
            Some(label) => Some(Select::create_label(label)),
            None => None,
        };
        // Create the select element
        let select = document()
            .create_element("select")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap();
        select.set_id("select");
        // Create and add the options elements to the select element
        for option in &self.options {
            let opt = document()
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            opt.set_inner_text(option);
            opt.set_value(option);
            select.append_child(&opt).unwrap();
        }

        match label {
            Some(label) => {
                wrapper.append_child(&label).unwrap();
            }
            None => {}
        }

        wrapper.append_child(&select).unwrap();
        wrapper.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Label for Select {
    fn create_label(label: &String) -> HtmlElement {
        // Create the label element
        let label_element = document()
            .create_element("label")
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap();
        label_element.set_inner_text(&label);
        label_element.set_html_for("select");

        label_element.dyn_into::<HtmlElement>().unwrap()
    }
}

impl<State> Connect<State> for Select {
    fn connect(&mut self, data: String) {
        self.label = Some(data);
    }

    // fn dispatch(&mut self, state: &State) {}
}

impl<State> ConnectedComponent<State> for Select {
    fn dispatch(&mut self, state: &State) {}
}

impl Select {
    pub fn new(label: Option<&str>, options: Vec<&str>) -> Select {
        let label = label.map(|label| label.to_owned());
        let options = options.iter().map(|s| s.to_string()).collect();

        Select { label, options }
    }
}
