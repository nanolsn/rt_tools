use super::{
    state::*,
};

#[derive(Debug)]
pub enum TileError<M, T> {
    StateError(StateError<M, T>),
    NoStates,
}

impl<M, T> From<StateError<M, T>> for TileError<M, T> {
    fn from(err: StateError<M, T>) -> Self { TileError::StateError(err) }
}

#[derive(Debug)]
pub struct Tile {
    pub states: Vec<State>,
    pub id: u32,
}

impl Tile {
    pub fn detect_state(&self) -> &State { &self.states[0] }
}
