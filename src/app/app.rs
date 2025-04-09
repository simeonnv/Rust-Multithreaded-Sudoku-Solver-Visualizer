use crate::state::state::State;

#[derive(Default)]
pub struct App {
    pub state: Option<State>,
}