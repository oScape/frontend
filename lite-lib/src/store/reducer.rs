/// A `Reducer` is a function which will apply an `Action` to a `State`
/// called previousState and output a `State` called `nextState`.
/// 
/// We don't mutate the state, we create a new Struct.
/// 
/// # Example
/// ```
/// struct State {
///    counter: i8
/// }
///
/// enum Action {
///     Increment,
///     Decrement
/// }
///
/// let reducer: Reducer<State, Action> = |state: State, action: Action| -> State {
///     match action {
///         Action::Increment => State {
///             counter: state.counter + 1
///         },
///         Action::Decrement => State {
///             counter: state.counter - 1
///         },
///         _ => State {
///             counter: state
///         },
///     }
/// };
/// ```
pub type Reducer<State, Action> = fn(State, Action) -> State;
