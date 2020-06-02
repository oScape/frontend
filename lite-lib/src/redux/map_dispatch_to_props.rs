pub trait MapDispatchToProps<State> {
    fn disptach(&self, state: &State);
}