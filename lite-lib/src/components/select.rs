use crate::component::{base::Base, label::Label};
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlLabelElement, HtmlOptionElement, HtmlSelectElement};

pub struct Select {
    label: String,
    parent: HtmlElement,
    options: Vec<String>,
}

impl Base for Select {
    fn render(&self) {
        // Insert element
        self.parent.append_child(&self.create_label()).unwrap();
        self.parent.append_child(&self.create_element()).unwrap();
    }

    fn create_element(&self) -> HtmlElement {
        // Create the select element
        let element = document()
            .create_element("select")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap();
        element.set_id("select");
        // Create and add the options elements to the select element
        for option in &self.options {
            let opt = document()
                .create_element("option")
                .unwrap()
                .dyn_into::<HtmlOptionElement>()
                .unwrap();
            opt.set_inner_text(option);
            opt.set_value(option);
            element.append_child(&opt).unwrap();
        }

        element.dyn_into::<HtmlElement>().unwrap()
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

impl Select {
    pub fn new(label: &str, parent: HtmlElement, options: Vec<&str>) -> Select {
        let label = label.to_owned();
        let options = options.iter().map(|s| s.to_string()).collect();

        Select {
            label,
            parent,
            options,
        }
    }
}
