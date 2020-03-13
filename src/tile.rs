use super::{
    state::{State, StateError},
    load::Load,
    resource::Resource,
    model::Model,
    parse::{
        tile::yaml_to_tile,
        YamlError,
        load_yaml_file,
    },
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

impl Load for Tile {
    const DIR: &'static str = "tiles";

    type Error = YamlError<TileError>;
    type Loader = Resource<Model>;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    {
        let yml = load_yaml_file(file)?;

        let res = yaml_to_tile(&yml, loader)
            .map_err(|e| YamlError::DataError(e))?;

        Ok(res)
    }
}
