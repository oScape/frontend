/// A `Subscription` is a closure which will be called by the `Store` when
/// the `State` will change.
///
/// # Example
/// ```
/// let mut store = Store::new(data_reducer, State::default());
///
/// let listener: Subscription<State> = |state: &State| {
///     log(&format!("Counter changed! New value: {}", state.data));
/// };
///
/// store.subscribe(listener);
/// ```
pub type Subscription<State> = fn(&State);
