use crate::component::*;
use crate::utils::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlOptionElement, HtmlSelectElement, HtmlLabelElement};

pub struct Select {
    label: String,
    parent: HtmlElement,
    options: Vec<String>,
}

impl Component for Select {
    fn render(&self) {
        self.parent.append_child(&self.create_element()).unwrap();
    }

    fn create_element(&self) -> HtmlElement {
        // Create the label element
        let label = document()
            .create_element(&self.label)
            .unwrap()
            .dyn_into::<HtmlLabelElement>()
            .unwrap();
        label.set_html_for("select");
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
