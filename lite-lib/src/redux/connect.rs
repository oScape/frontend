pub trait Connect<State> {
    fn connect(&mut self, data: String);

    // fn dispatch(&self, state: &State) {}

    // fn dispatch(&mut self, state: &State);
}
