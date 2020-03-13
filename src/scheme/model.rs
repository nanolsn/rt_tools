use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(super) struct Model {
    pos: Option<Vec<[f32; 3]>>,
    st: Option<Vec<[f32; 2]>>,
    norm: Option<Vec<[f32; 3]>>,
    faces: Option<Vec<Face>>,
    full_sides: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(super) struct Face {
    pos: Option<Vec<[f32; 3]>>,
    st: Option<Vec<[f32; 2]>>,
    norm: Option<[f32; 3]>,
    layer: Option<u32>,
    contact: Option<String>,
    data: Option<Data>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(super) struct Data {
    pos: Option<Vec<u32>>,
    st: Option<Vec<u32>>,
    norm: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_empty() {
        let code = "{}";
        let model: Model = serde_yaml::from_str(code).unwrap();

        assert_eq!(model, Model {
            pos: None,
            st: None,
            norm: None,
            faces: None,
            full_sides: None,
        });
    }

    #[test]
    fn deserialize() {
        let code = r#"
        pos:
          - [ 1.0, 0.5, 1.0 ]
          - [ 1.0, 0.0, 1.0 ]
        st:
          - [ 0.0, 1.0 ]
          - [ 0.0, 0.0 ]
        norm: [[ 1.0, 0.0, 0.0 ]]
        faces:
          - data:
                pos: [ 0, 1, 0 ]
                st: [ 0, 1, 1 ]
                norm: 0

          - pos:
              - [ 1.0, 0.0, -1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, -1.0 ]
            st:
              - [ 0, 1 ]
              - [ 0, 0 ]
              - [ 1, 0 ]
              - [ 1, 1 ]
            norm: [ 0, 1, 0 ]
            layer: 6
            contact: lr

          - pos:
              - [ 1.0, 0.0, -1.0 ]
              - [ 1.0, 0.0, 1.0 ]
              - [ 1.0, 0.0, 1.0 ]
            st:
              - [ 0, 1 ]
              - [ 0, 0 ]
              - [ 1, 0 ]
            norm: [ 0, 0, 1 ]
            layer: 5
            contact: fb
        full_sides: ud
        "#;

        let model: Model = serde_yaml::from_str(code).unwrap();

        assert_eq!(model, Model {
            pos: Some(vec![
                [1.0, 0.5, 1.0],
                [1.0, 0.0, 1.0],
            ]),
            st: Some(vec![
                [0.0, 1.0],
                [0.0, 0.0],
            ]),
            norm: Some(vec![[1.0, 0.0, 0.0]]),
            faces: Some(vec![
                Face {
                    pos: None,
                    st: None,
                    norm: None,
                    layer: None,
                    contact: None,
                    data: Some(Data {
                        pos: Some(vec![0, 1, 0]),
                        st: Some(vec![0, 1, 1]),
                        norm: Some(0),
                    }),
                },
                Face {
                    pos: Some(vec![
                        [1.0, 0.0, -1.0],
                        [1.0, 0.0, 1.0],
                        [1.0, 0.0, 1.0],
                        [1.0, 0.0, -1.0],
                    ]),
                    st: Some(vec![
                        [0.0, 1.0],
                        [0.0, 0.0],
                        [1.0, 0.0],
                        [1.0, 1.0],
                    ]),
                    norm: Some([0.0, 1.0, 0.0]),
                    layer: Some(6),
                    contact: Some("lr".to_owned()),
                    data: None,
                },
                Face {
                    pos: Some(vec![
                        [1.0, 0.0, -1.0],
                        [1.0, 0.0, 1.0],
                        [1.0, 0.0, 1.0],
                    ]),
                    st: Some(vec![
                        [0.0, 1.0],
                        [0.0, 0.0],
                        [1.0, 0.0],
                    ]),
                    norm: Some([0.0, 0.0, 1.0]),
                    layer: Some(5),
                    contact: Some("fb".to_owned()),
                    data: None,
                },
            ]),
            full_sides: Some("ud".to_owned()),
        });
    }
}
