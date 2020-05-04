pub struct Store {
    data: Vec<String>,
}

impl Store {
    pub fn new() -> Store {
        let data = vec!["yolo".to_owned()];
        Store { data }
    }

    pub fn add_data(&mut self, data: String) {
        self.data.push(data);
    }

    pub fn get_first_data(self) -> String {
        self.data.first().unwrap().to_owned()
    }
}
