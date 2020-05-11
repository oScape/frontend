use crate::component::{Base, Label};
use crate::store::provider::UnknowInstance;
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlLabelElement, HtmlOptionElement, HtmlSelectElement};

pub struct Select {
    label: String,
    options: Vec<String>,
    _parent: UnknowInstance,
}

impl Base for Select {
    fn render(&self) {
        document()
            .body()
            .unwrap()
            .append_child(&self.create_label())
            .unwrap();
        document()
            .body()
            .unwrap()
            .append_child(&self.create_element())
            .unwrap();
        // self.parent.append_child(&self.create_label()).unwrap();
        // self.parent.append_child(&self.create_element()).unwrap();
    }

    fn create_element(&self) -> HtmlElement {
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

        select.dyn_into::<HtmlElement>().unwrap()
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
    pub fn new(label: &str, options: Vec<&str>, parent: UnknowInstance) -> Select {
        let label = label.to_owned();
        let options = options.iter().map(|s| s.to_string()).collect();

        Select {
            label,
            options,
            _parent: parent,
        }
    }
}
