pub trait Connect<State> {
    fn connect(&self, state: &State);
}
