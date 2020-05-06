use super::reducer::Reducer;
use super::subscription::Subscription;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Store {
    data: Vec<String>,
    reducers: Vec<Reducer>,
    subscriptions: Vec<Subscription>,
}

static STORE: Lazy<Arc<Mutex<Store>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Store {
        data: vec!["hard coded".to_owned()],
        reducers: Vec::new(),
        subscriptions: Vec::new(),
    }))
});

impl Store {
    pub fn get_store() -> &'static Arc<Mutex<Store>> {
        &*STORE
    }

    pub fn set_data(data: String) {
        let store = &mut *STORE.lock().unwrap();
        &store.data.push(data);
        for sub in &store.subscriptions {
            sub();
        }
    }

    pub fn get_values() -> Vec<String> {
        let store = &mut *STORE.lock().unwrap();
        store.data.clone()
    }

    pub fn subscribe(sub: Subscription) {
        let store = &mut *STORE.lock().unwrap();
        store.subscriptions.push(sub);
    }
}
