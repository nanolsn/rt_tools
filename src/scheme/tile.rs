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
};

// TODO: Make Resource<Model> loader instead
pub struct Loader;

impl Loader {
    pub fn load(&self, _: &str) -> usize { 0 }
}

fn convert(src: Tile, loader: Loader) -> Result<tl::Tile, tl::TileError> {
    let models = src.models.unwrap_or_default();
    let textures = src.textures.unwrap_or_default();
    let states = src.states.unwrap_or_default();

    let convert_state = |state: State| {
        Ok(st::State {
            model: {
                let model = state.model.ok_or(st::StateError::NoModelDefined)?;

                let model_file = models.get(model as usize)
                    .ok_or(st::StateError::OutOfRange)?;

                loader.load(&*model_file)
            },

            shell: {
                let actions_result: Result<Vec<ShellTransformAction>, st::StateError> = state
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

                // TODO: Load textures

                layers
            },
        })
    };

    let states_result: Result<Vec<st::State>, st::StateError> = states
        .into_iter()
        .map(convert_state)
        .collect();

    Ok(tl::Tile {
        states: states_result?,
        id: 0,
    })
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
