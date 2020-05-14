use crate::component::{Component, Label};
use crate::store::{connect::Connect, provider::ConnectedComponent};
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlDivElement, HtmlElement, HtmlLabelElement, HtmlOptionElement, HtmlSelectElement,
};

pub struct Select {
    label: String,
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
        let label = self.create_label();
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

        wrapper.append_child(&label).unwrap();
        wrapper.append_child(&select).unwrap();
        wrapper.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Label for Select {
    fn create_label(&self) -> HtmlElement {
        // Create the label element
        let label = document()
            .create_element("label")
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap();
        label.set_inner_text(&self.label);
        label.set_html_for("select");

        label.dyn_into::<HtmlElement>().unwrap()
    }
}

impl Connect for Select {
    fn connect(&mut self, data: String) {
        self.label = data;
    }
}

impl ConnectedComponent for Select {}

impl Select {
    pub fn new(label: &str, options: Vec<&str>) -> Select {
        let label = label.to_owned();
        let options = options.iter().map(|s| s.to_string()).collect();

        Select { label, options }
    }
}
