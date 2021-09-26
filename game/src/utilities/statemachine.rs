use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct StateMachine<State, Data>
where
    State: Eq + Hash + Clone + Display + Default,
{
    default_state: State,
    callback_map: HashMap<State, Box<dyn Fn(&mut Data)>>,
}

impl<State, Data> StateMachine<State, Data>
where
    State: Eq + Hash + Clone + Display + Default,
{
    /// Construct a new StateMachine
    pub fn new() -> Self {
        Self {
            default_state: State::default(),
            callback_map: HashMap::new(),
        }
    }

    /// Override the default state function
    pub fn set_default_handler(&mut self, callback: Box<dyn Fn(&mut Data)>) {
        self.callback_map.insert(self.default_state.clone(), callback);
    }

    /// Add a new state function
    pub fn add_state(&mut self, state: State, callback: Box<dyn Fn(&mut Data)>) {
        self.callback_map.insert(state, callback);
    }
}
