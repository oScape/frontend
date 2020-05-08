/// A `Reducer` is a closure which will apply an `Action` to a `State`
/// called previousState and output a `State` called `nextState`.
/// 
/// We don't mutate the state, we create a new Struct.
/// 
/// # Example
/// ```
/// #[derive(Default)]
/// struct State {
///     data: String
/// }
///
/// enum Action {
///     Change(String)
/// }
///
/// fn data_reducer(state: &State, action: &Action) -> State {
///     match action {
///         Action::Change(data) => State {
///             data: data.clone()
///         }
///     }
/// }
/// ```
pub type Reducer<State, Action> = fn(&State, &Action) -> State;
