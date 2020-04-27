use crate::component::*;
use crate::utils::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlOptionElement, HtmlSelectElement};

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
        let element = document()
            .create_element("select")
            .unwrap()
            .dyn_into::<HtmlSelectElement>()
            .unwrap();
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
