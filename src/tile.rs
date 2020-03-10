use super::{
    state::{State, StateError},
    load::{LoadDir, Load},
    resource::Resource,
    model::Model,
    parse::tile::yaml_to_tile,
};

#[derive(Debug)]
pub enum TileError {
    ScanError(yaml::ScanError),
    FormatError,
    StateError(StateError),
    NoStates,
}

impl From<yaml::ScanError> for TileError {
    fn from(err: yaml::ScanError) -> Self { TileError::ScanError(err) }
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

impl LoadDir for Tile {
    const DIR: &'static str = "tiles";
}

impl Load for Tile {
    type Error = TileError;
    type Loader = Resource<Model>;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    {
        let path = file.as_ref().to_string_lossy();
        let ls = yaml::YamlLoader::load_from_str(path.as_ref())?;

        if ls.len() != 1 { Err(TileError::FormatError)? }

        let yml = ls.into_iter().next().unwrap();
        Ok(yaml_to_tile(&yml, loader)?)
    }
}
