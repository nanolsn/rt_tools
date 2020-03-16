use super::{
    state::*,
};

#[derive(Debug, Eq, PartialEq)]
pub enum TileField {
    Models,
    Textures,
    States,
}

impl TileField {
    pub fn path(&self) -> &str {
        match self {
            TileField::Models => "models",
            TileField::Textures => "textures",
            TileField::States => "states",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TileError<M, T> {
    StateError(StateError<M, T>),
    NoStates,
}

impl<M, T> From<StateError<M, T>> for TileError<M, T> {
    fn from(err: StateError<M, T>) -> Self { TileError::StateError(err) }
}

impl<M, T> super::error::Error for TileError<M, T> {
    fn title() -> &'static str { "Tile Error" }

    fn case(&self) -> &str {
        match self {
            TileError::StateError(s) => s.case(),
            TileError::NoStates => "No States",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            TileError::StateError(s) => s.clarification(),
            TileError::NoStates => None,
        }
    }
}

impl<M, T> std::fmt::Display for TileError<M, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", super::error::Error::display(self))
    }
}

#[derive(Debug, PartialEq)]
pub struct Tile {
    pub states: Vec<State>,
    pub id: u32,
}

impl Tile {
    pub fn detect_state(&self) -> &State { &self.states[0] }
}
