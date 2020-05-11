use crate::component::Base;
use crate::store::provider::UnknowInstance;
use crate::utils::dom::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlFormElement, HtmlInputElement, HtmlLabelElement};

pub enum FormElementType {
    // Button,
    // Fieldset,
    Input,
    // Select,
    // Textarea
}

pub struct FormElement {
    name: String,
    form_element_type: FormElementType,
    label: Option<String>,
}

pub struct Form {
    name: String,
    form_elements: Vec<FormElement>,
    _parent: UnknowInstance,
}

impl Base for Form {
    fn render(&self) {
        document()
            .body()
            .unwrap()
            .append_child(&self.create_element())
            .unwrap();
        // self.parent.append_child(&self.create_element()).unwrap();
    }

    fn create_element(&self) -> HtmlElement {
        // Create the form element
        let form = document()
            .create_element("form")
            .unwrap()
            .dyn_into::<HtmlFormElement>()
            .unwrap();
        form.set_id("form");
        form.set_name(&self.name);

        for form_element in &self.form_elements {
            match form_element.form_element_type {
                FormElementType::Input => {
                    match &form_element.label {
                        Some(string) => {
                            let label = document()
                                .create_element("label")
                                .unwrap()
                                .dyn_into::<HtmlLabelElement>()
                                .unwrap();
                            label.set_inner_text(string);
                            label.set_html_for(&form_element.name);
                            form.append_child(&label).unwrap();
                        }
                        None => (),
                    }
                    let input = document()
                        .create_element("input")
                        .unwrap()
                        .dyn_into::<HtmlInputElement>()
                        .unwrap();
                    input.set_type("text");
                    input.set_id(&form_element.name);
                    input.set_name(&form_element.name);
                    form.append_child(&input).unwrap();
                }
            }
        }

        form.dyn_into::<HtmlElement>().unwrap()
    }
}

impl FormElement {
    pub fn new(name: &str, form_element_type: FormElementType, label: Option<&str>) -> FormElement {
        let name = name.to_owned();
        let label: Option<String> = label.map(String::from);

        FormElement {
            name,
            form_element_type,
            label,
        }
    }
}

impl Form {
    pub fn new(name: &str, form_elements: Vec<FormElement>, parent: UnknowInstance) -> Form {
        let name = name.to_owned();

        Form {
            name,
            form_elements,
            _parent: parent,
        }
    }
}
