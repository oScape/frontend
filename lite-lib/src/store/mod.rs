struct Store {
    data: Vec<String>,
}

impl Store {
    fn new() -> Store {
        let data = vec!["yolo".to_owned()];
        Store { data }
    }
}

lazy_static! {
    static ref STORE: Store = { Store::new() };
}
