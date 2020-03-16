use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Model {
    pos: Option<Vec<[f32; 3]>>,
    st: Option<Vec<[f32; 2]>>,
    norm: Option<Vec<[f32; 3]>>,
    faces: Option<Vec<Face>>,
    full_sides: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Face {
    pos: Option<Vec<[f32; 3]>>,
    st: Option<Vec<[f32; 2]>>,
    norm: Option<[f32; 3]>,
    layer: Option<u32>,
    contact: Option<String>,
    data: Option<Data>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct Data {
    pos: Option<Vec<u32>>,
    st: Option<Vec<u32>>,
    norm: Option<u32>,
}

use crate::{
    model as md,
    face as fc,
    vertex::Vertex,
    normal::calc_normal,
};

fn convert(src: Model) -> Result<md::Model, md::ModelError> {
    let pos = src.pos.unwrap_or_default();
    let st = src.st.unwrap_or_default();
    let norm = src.norm.unwrap_or_default();
    let faces = src.faces.unwrap_or_default();
    let full_sides = src.full_sides.unwrap_or_default();

    let faces_result: Result<Vec<fc::Face>, fc::FaceError> = faces
        .into_iter()
        .map(|f| {
            if f.data.is_some() && (f.pos.is_some() || f.st.is_some() || f.norm.is_some()) {
                Err(fc::FaceError::IncorrectDataFormat)?
            }

            let vertexes: Vec<Vertex> = if let Some(d) = f.data {
                let pos_ids = d.pos
                    .ok_or(fc::FaceError::WrongVertexNumber(fc::FaceField::DataPos))?;

                let st_ids = d.st
                    .ok_or(fc::FaceError::WrongVertexNumber(fc::FaceField::DataSt))?;

                if pos_ids.len() != 3 && pos_ids.len() != 4 {
                    Err(fc::FaceError::WrongVertexNumber(fc::FaceField::DataPos))?
                }

                if st_ids.len() != 3 && st_ids.len() != 4 {
                    Err(fc::FaceError::WrongVertexNumber(fc::FaceField::DataSt))?
                }

                let norm = d
                    .norm
                    .and_then(|idx| norm.get(idx as usize))
                    .map(|&[q, w, e]| Ok(glm::vec3(q, w, e)))
                    .unwrap_or_else(|| {
                        let i = pos_ids[0] as usize;
                        let j = pos_ids[1] as usize;
                        let k = pos_ids[2] as usize;

                        let &a = pos.get(i)
                            .ok_or(fc::FaceError::OutOfRange(fc::FaceField::Pos, i))?;

                        let &b = pos.get(j)
                            .ok_or(fc::FaceError::OutOfRange(fc::FaceField::Pos, j))?;

                        let &c = pos.get(k)
                            .ok_or(fc::FaceError::OutOfRange(fc::FaceField::Pos, k))?;

                        Ok(calc_normal(a, b, c))
                    })?;

                let res: Result<Vec<Vertex>, fc::FaceError> = pos_ids
                    .into_iter()
                    .zip(st_ids)
                    .map(|(pos_id, st_id)| ((pos_id as usize, st_id as usize)))
                    .map(|(pos_id, st_id)| {
                        let &[x, y, z] = pos
                            .get(pos_id)
                            .ok_or(fc::FaceError::OutOfRange(fc::FaceField::Pos, pos_id))?;

                        let &[s, t] = st
                            .get(st_id)
                            .ok_or(fc::FaceError::OutOfRange(fc::FaceField::St, st_id))?;

                        Ok(Vertex {
                            pos: glm::vec3(x, y, z),
                            st: glm::vec2(s, t),
                            norm,
                        })
                    })
                    .collect();

                res?
            } else {
                let pos = f.pos
                    .ok_or(fc::FaceError::WrongVertexNumber(fc::FaceField::Pos))?;

                let st = f.st
                    .ok_or(fc::FaceError::WrongVertexNumber(fc::FaceField::St))?;

                let norm = f
                    .norm
                    .map(|[q, w, e]| glm::vec3(q, w, e))
                    .unwrap_or_else(|| calc_normal(pos[0], pos[1], pos[2]));

                pos
                    .into_iter()
                    .zip(st)
                    .map(|([x, y, z], [s, t])|
                        Vertex {
                            pos: glm::vec3(x, y, z),
                            st: glm::vec2(s, t),
                            norm,
                        })
                    .collect()
            };

            Ok(fc::Face {
                vertexes: fc::FaceVertexes::from_slice(&*vertexes).unwrap(),
                contact: f.contact.unwrap_or_default().as_str().into(),
                layer: f.layer.unwrap_or_default(),
            })
        })
        .collect();

    Ok(md::Model {
        faces: faces_result?,
        full_sides: full_sides.as_str().into(),
    })
}

impl super::ConvertFrom<Model, ()> for md::Model {
    type Error = md::ModelError;

    fn convert(from: Model, _: ()) -> Result<Self, Self::Error> { convert(from) }
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

    use crate::sides::*;

    #[test]
    fn convert_empty() {
        let model = Model {
            pos: None,
            st: None,
            norm: None,
            faces: None,
            full_sides: None,
        };

        let expected = md::Model {
            faces: vec![],
            full_sides: Sides::empty(),
        };

        assert_eq!(super::convert(model), Ok(expected));
    }

    #[test]
    fn convert_err() {
        let model = Model {
            pos: Some(vec![
                [0.0, 1.0, 2.0],
            ]),
            st: Some(vec![
                [0.0, 1.0],
            ]),
            norm: Some(vec![
                [0.0, 1.0, 0.0],
            ]),
            faces: Some(vec![
                Face {
                    pos: None,
                    st: None,
                    norm: None,
                    layer: None,
                    contact: None,
                    data: Some(Data {
                        pos: Some(vec![1, 1, 1]),
                        st: Some(vec![0, 0, 0]),
                        norm: Some(0),
                    }),
                },
            ]),
            full_sides: None,
        };

        let err = md::ModelError::FaceError(fc::FaceError::OutOfRange(
            fc::FaceField::Pos,
            1,
        ));

        assert_eq!(super::convert(model), Err(err));

        let model = Model {
            pos: None,
            st: None,
            norm: None,
            faces: Some(vec![
                Face {
                    pos: Some(vec![
                        [0.0, 1.0, 0.0],
                    ]),
                    st: None,
                    norm: None,
                    layer: None,
                    contact: None,
                    data: Some(Data {
                        pos: Some(vec![1, 1, 1]),
                        st: Some(vec![0, 0, 0]),
                        norm: Some(0),
                    }),
                },
            ]),
            full_sides: None,
        };

        let err = md::ModelError::FaceError(fc::FaceError::IncorrectDataFormat);
        assert_eq!(super::convert(model), Err(err));
    }

    #[test]
    fn convert() {
        use glm::{vec2, vec3};

        let model = Model {
            pos: Some(vec![
                [0.0, 1.0, 2.0],
                [1.0, 1.0, 0.0],
                [2.0, 1.0, 0.0],
            ]),
            st: Some(vec![
                [0.0, 1.0],
                [0.0, 0.0],
                [1.0, 0.0],
            ]),
            norm: Some(vec![
                [0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0],
            ]),
            faces: Some(vec![
                Face {
                    pos: None,
                    st: None,
                    norm: None,
                    layer: None,
                    contact: None,
                    data: Some(Data {
                        pos: Some(vec![0, 1, 2]),
                        st: Some(vec![0, 1, 2]),
                        norm: Some(0),
                    }),
                },
                Face {
                    pos: None,
                    st: None,
                    norm: None,
                    layer: Some(2),
                    contact: None,
                    data: Some(Data {
                        pos: Some(vec![1, 0, 1, 0]),
                        st: Some(vec![1, 0, 1, 0]),
                        norm: Some(1),
                    }),
                },
                Face {
                    pos: Some(vec![
                        [1.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 0.0],
                    ]),
                    st: Some(vec![
                        [0.0, 0.0],
                        [0.0, 1.0],
                        [1.0, 1.0],
                        [1.0, 0.0],
                    ]),
                    norm: Some([0.0, 0.0, 1.0]),
                    layer: None,
                    contact: Some("ud".to_owned()),
                    data: None,
                },
            ]),
            full_sides: Some(".".to_owned()),
        };

        let expected = md::Model {
            faces: vec![
                fc::Face {
                    vertexes: fc::FaceVertexes::Triangle([
                        Vertex {
                            pos: vec3(0.0, 1.0, 2.0),
                            st: vec2(0.0, 1.0),
                            norm: vec3(0.0, 1.0, 0.0),
                        },
                        Vertex {
                            pos: vec3(1.0, 1.0, 0.0),
                            st: vec2(0.0, 0.0),
                            norm: vec3(0.0, 1.0, 0.0),
                        },
                        Vertex {
                            pos: vec3(2.0, 1.0, 0.0),
                            st: vec2(1.0, 0.0),
                            norm: vec3(0.0, 1.0, 0.0),
                        },
                    ]),
                    contact: Sides::empty(),
                    layer: 0,
                },
                fc::Face {
                    vertexes: fc::FaceVertexes::Square([
                        Vertex {
                            pos: vec3(1.0, 1.0, 0.0),
                            st: vec2(0.0, 0.0),
                            norm: vec3(0.0, -1.0, 0.0),
                        },
                        Vertex {
                            pos: vec3(0.0, 1.0, 2.0),
                            st: vec2(0.0, 1.0),
                            norm: vec3(0.0, -1.0, 0.0),
                        },
                        Vertex {
                            pos: vec3(1.0, 1.0, 0.0),
                            st: vec2(0.0, 0.0),
                            norm: vec3(0.0, -1.0, 0.0),
                        },
                        Vertex {
                            pos: vec3(0.0, 1.0, 2.0),
                            st: vec2(0.0, 1.0),
                            norm: vec3(0.0, -1.0, 0.0),
                        },
                    ]),
                    contact: Sides::empty(),
                    layer: 2,
                },
                fc::Face {
                    vertexes: fc::FaceVertexes::Square([
                        Vertex {
                            pos: vec3(1.0, 0.0, 0.0),
                            st: vec2(0.0, 0.0),
                            norm: vec3(0.0, 0.0, 1.0),
                        },
                        Vertex {
                            pos: vec3(0.0, 1.0, 0.0),
                            st: vec2(0.0, 1.0),
                            norm: vec3(0.0, 0.0, 1.0),
                        },
                        Vertex {
                            pos: vec3(0.0, 0.0, 1.0),
                            st: vec2(1.0, 1.0),
                            norm: vec3(0.0, 0.0, 1.0),
                        },
                        Vertex {
                            pos: vec3(0.0, 0.0, 0.0),
                            st: vec2(1.0, 0.0),
                            norm: vec3(0.0, 0.0, 1.0),
                        },
                    ]),
                    contact: Up | Down,
                    layer: 0,
                },
            ],
            full_sides: Sides::all(),
        };

        assert_eq!(super::convert(model), Ok(expected));
    }
}
