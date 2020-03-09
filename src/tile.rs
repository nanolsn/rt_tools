use super::{
    state::{State, StateError},
};

#[derive(Debug)]
pub enum TileError {
    StateError(StateError),
    NoStates,
}

impl From<StateError> for TileError {
    fn from(err: StateError) -> Self { TileError::StateError(err) }
}

#[derive(Debug)]
pub struct Tile {
    pub states: Vec<State>,
    pub id: u32,
}

impl Tile {
    pub fn detect_state(&self) -> &State { &self.states[0] }
}
