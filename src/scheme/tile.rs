use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Tile {
    models: Option<Vec<String>>,
    textures: Option<Vec<String>>,
    states: Option<Vec<State>>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct State {
    model: Option<u32>,
    layers: Option<Vec<u32>>,
    transform: Option<Vec<String>>,
}

use crate::{
    tile as tl,
    state as st,
    shell_transform::{Shell, ShellTransformAction, apply_actions},
    resource::Resource,
    load::Load,
};

type TileLoaders<M, T> = (Resource<M>, Resource<T>);
type TileResult<M, T> = Result<tl::Tile, tl::TileError<M, T>>;

fn convert<M, T>(src: Tile, loaders: &mut TileLoaders<M, T>) -> TileResult<M::Error, T::Error>
    where
        M: Load<Loader=()>,
        T: Load<Loader=()>,
{
    let (model_loader, texture_loader) = loaders;

    let models = src.models.unwrap_or_default();
    let textures = src.textures.unwrap_or_default();
    let states = src.states.unwrap_or_default();

    let convert_state = |state: State| {
        Ok(st::State {
            model: {
                let model = state.model.ok_or(st::StateError::NoModelDefined)?;

                let model_file = models.get(model as usize)
                    .ok_or(st::StateError::OutOfRange)?;

                model_loader.load(&*model_file)
                    .map_err(|e| st::StateError::ModelError(e))?
            },

            shell: {
                let actions_result: Result<Vec<ShellTransformAction>, st::StateError<_, _>> = state
                    .transform
                    .unwrap_or_default()
                    .into_iter()
                    .map(|s| {
                        use std::convert::TryFrom;

                        ShellTransformAction::try_from(&*s)
                            .map_err(|_| st::StateError::TransformError)
                    })
                    .collect();

                *apply_actions(&mut Shell::new(), actions_result?)
            },

            layers: {
                let layers = state.layers.ok_or(st::StateError::NoLayerDefined)?;

                if layers.iter().any(|&l| l as usize >= textures.len()) {
                    Err(st::StateError::OutOfRange)?
                }

                let layers_result: Result<Vec<u32>, _> = layers
                    .into_iter()
                    .map(|l| {
                        let texture_file = textures
                            .get(l as usize)
                            .ok_or(st::StateError::OutOfRange)?;

                        texture_loader.load(&*texture_file)
                            .map_err(|e| st::StateError::TextureError(e))
                            .map(|id| id as u32)
                    })
                    .collect();

                layers_result?
            },
        })
    };

    let states_result: Result<Vec<st::State>, st::StateError<_, _>> = states
        .into_iter()
        .map(convert_state)
        .collect();

    Ok(tl::Tile {
        states: states_result?,
        id: 0,
    })
}

impl<M, T> super::ConvertFrom<Tile, TileLoaders<M, T>> for tl::Tile
    where
        M: Load<Loader=()>,
        T: Load<Loader=()>,
{
    type Error = tl::TileError<M::Error, T::Error>;

    fn convert(from: Tile, loader: &mut TileLoaders<M, T>) -> TileResult<M::Error, T::Error> {
        convert(from, loader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_empty() {
        let code = "{}";
        let tile: Tile = serde_yaml::from_str(code).unwrap();

        assert_eq!(tile, Tile {
            models: None,
            textures: None,
            states: None,
        });
    }

    #[test]
    fn deserialize() {
        let code = r#"
        models:
        -   m1.yml
        -   m2.yml
        textures:
        -   1.png
        -   2.png
        -   3.png
        states:
        -   model: 1
            layers: [ 1, 1, 1 ]
            transform:
                -   flip_x
                -   turn_y
                -   turn_-z
        -   model: 2
        "#;

        let tile: Tile = serde_yaml::from_str(code).unwrap();

        assert_eq!(tile, Tile {
            models: Some(vec![
                "m1.yml".to_owned(),
                "m2.yml".to_owned(),
            ]),
            textures: Some(vec![
                "1.png".to_owned(),
                "2.png".to_owned(),
                "3.png".to_owned(),
            ]),
            states: Some(vec![
                State {
                    model: Some(1),
                    layers: Some(vec![1, 1, 1]),
                    transform: Some(vec![
                        "flip_x".to_owned(),
                        "turn_y".to_owned(),
                        "turn_-z".to_owned(),
                    ]),
                },
                State {
                    model: Some(2),
                    layers: None,
                    transform: None,
                },
            ]),
        });
    }
}
