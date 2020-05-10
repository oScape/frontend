use crate::utils::query_selector::{query_selector, SelectorType};
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::EventTarget;

pub struct EventListener {
    _target_event: EventTarget,
    _event_type: String,
    _callback: fn(),
}

impl EventListener {
    pub fn new(
        selector_type: SelectorType,
        selector: &str,
        event_type: &str,
        callback: fn(),
    ) -> EventListener {
        let target_element = query_selector(selector_type, selector);
        let target_event = EventTarget::from(target_element);
        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);

        target_event
            .add_event_listener_with_callback(
                event_type,
                closure.as_ref().to_owned().unchecked_ref::<Function>(),
            )
            .unwrap();
        closure.forget();

        let event_type = event_type.to_owned();

        EventListener {
            _target_event: target_event,
            _event_type: event_type,
            _callback: callback,
        }
    }
}
