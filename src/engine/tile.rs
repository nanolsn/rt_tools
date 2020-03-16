use super::state::State;

#[derive(Debug, PartialEq)]
pub struct Tile {
    pub states: Vec<State>,
    pub id: u32,
}

impl Tile {
    pub fn detect_state(&self) -> &State { &self.states[0] }
}
