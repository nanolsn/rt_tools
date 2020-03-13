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
