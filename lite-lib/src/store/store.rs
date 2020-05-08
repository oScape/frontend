use super::reducer::Reducer;
use super::subscription::Subscription;

/// A container holding a state and providing the possibility to dispatch actions.
///
/// A store is defined by the state is holds and the actions it can dispatch.
pub struct Store<State, Action> {
    reducer: Reducer<State, Action>,
    state: State,
    subscriptions: Vec<Subscription<State>>
}

impl<State, Action> Store<State, Action> {
    /// Create a store
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
    ///
    /// let mut store = Store::new(data_reducer, State::default());
    /// ```
    pub fn new(reducer: Reducer<State, Action>, initial_state: State) -> Self {
        Self {
            reducer,
            state: initial_state,
            subscriptions: Vec::new()
        }
    }

    /// Return the current state.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Dispatch an action which is executed by the reducer.
    ///
    /// # Example
    /// ```
    /// store.dispatch(Action::Change(ouput));
    /// ```
    pub fn dispatch(&mut self, action: Action) {
        self.dispatch_reducer(&action);
    }

    /// Execute the reducer.
    fn dispatch_reducer(&mut self, action: &Action) {
        self.state = (&self.reducer)(self.state(), action);
        self.dispatch_subscriptions();
    }

    /// Execute the subscription.
    fn dispatch_subscriptions(&self) {
        for subscription in &self.subscriptions {
            subscription(self.state());
        }
    }

    /// Subscribe a callback to the store which is executed at any changes of state
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
    pub fn subscribe(&mut self, callback: Subscription<State>) {
        self.subscriptions.push(callback);
    }
}