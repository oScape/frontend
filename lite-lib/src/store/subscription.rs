/// A `Subscription` is a closure which will be called by the `Store` when
/// the `State` will change.
///
/// # Example
/// ```
/// let listener = |state: State| {
///     println!("The counter as been incremented: {}", state.counter);
/// };
///
/// Store::subscribe(listener);
///
/// Store::dispatch(Action::Increment);
/// ```
pub type Subscription<State> = fn(State);
