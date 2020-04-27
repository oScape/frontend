use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{EventTarget, HtmlElement};

pub struct EventListener {
    target: EventTarget,
    event_type: String,
    callback: fn(),
}

impl EventListener {
    pub fn new(target: HtmlElement, event_type: &str, callback: fn()) -> EventListener {
        let target = EventTarget::from(target);
        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn()>);

        target
            .add_event_listener_with_callback(
                event_type,
                closure.as_ref().to_owned().unchecked_ref::<Function>(),
            )
            .unwrap();
        closure.forget();

        let event_type = event_type.to_owned();

        EventListener {
            target,
            event_type,
            callback,
        }
    }
}
