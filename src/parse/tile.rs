use super::{
    super::{
        tile::{Tile, TileError},
        state::StateError,
        model::Model,
        resource::Resource,
    },
    state::yaml_to_state,
    parse,
};

pub fn yaml_to_tile(yml: &yaml::Yaml, res: &mut Resource<Model>) -> Result<Tile, TileError> {
    let models: Vec<String> = parse(&yml["models"])
        .map_err(|_| StateError::NoModelDefined)?;

    let arr = yml["states"].as_vec().ok_or(TileError::NoStates)?;
    let mut states = Vec::with_capacity(arr.len());

    for y in arr {
        states.push(yaml_to_state(y, res, &*models)?);
    }

    Ok(Tile { states, id: 0 })
}
