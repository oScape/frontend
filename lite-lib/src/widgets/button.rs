use crate::utils::*;

pub fn create() {
    let document = get_document();
    let body = get_body();

    let element = document.create_element("p").expect("could not create element");
    element.set_inner_html("Hello from lite-lib");

    body.append_child(&element).expect("could not insert element");
}