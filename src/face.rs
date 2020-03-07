use std::convert::{TryFrom, TryInto};

use super::{
    sides::Sides,
    vertex::Vertex,
    parse::Parse,
};

#[derive(Debug)]
pub enum FaceError {
    IncorrectVertexNumber,
    Vec3Error,
    Vec2Error,
    SidesError,
}

#[derive(Debug, PartialEq)]
pub enum FaceVertexes {
    Triangle([Vertex; 3]),
    Square([Vertex; 4]),
}

impl TryFrom<&[Vertex]> for FaceVertexes {
    type Error = FaceError;

    fn try_from(vs: &[Vertex]) -> Result<Self, Self::Error> {
        Ok(match vs.len() {
            3 => FaceVertexes::Triangle([vs[0], vs[1], vs[2]]),
            4 => FaceVertexes::Square([vs[0], vs[1], vs[2], vs[3]]),
            _ => Err(FaceError::IncorrectVertexNumber)?,
        })
    }
}

impl FaceVertexes {
    pub fn extend_vertexes(&self, vertexes: &mut Vec<Vertex>, indexes: &mut Vec<u32>) {
        let index = indexes.len() as u32;

        match self {
            FaceVertexes::Triangle(vs) => {
                vertexes.extend(vs);
                indexes.extend(&[
                    index,
                    index + 1,
                    index + 2,
                ]);
            }
            FaceVertexes::Square(vs) => {
                vertexes.extend(vs);
                indexes.extend(&[
                    index,
                    index + 1,
                    index + 2,
                    index,
                    index + 2,
                    index + 3,
                ]);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Face {
    pub vertexes: FaceVertexes,
    pub contact: Sides,
    pub layer: u32,
}

impl Face {
    pub fn new(vertexes: &[Vertex], contact: Sides, layer: u32) -> Result<Self, FaceError> {
        Ok(Face { vertexes: vertexes.try_into()?, contact, layer })
    }
}

impl Parse for Face {
    type DataError = FaceError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let layer = yml["layer"].as_i64().unwrap_or_default() as u32;

        let contact: Sides = Parse::parse(&yml["contact"])
            .map_err(|_| FaceError::SidesError)?;

        let normal: glm::Vec3 = Parse::parse(&yml["normal"])
            .map_err(|_| FaceError::Vec3Error)?;

        let pos: Vec<glm::Vec3> = Parse::parse(&yml["positions"])
            .map_err(|_| FaceError::Vec3Error)?;

        let st: Vec<glm::Vec2> = Parse::parse(&yml["st"])
            .map_err(|_| FaceError::Vec2Error)?;

        let vs: Vec<Vertex> = pos
            .iter()
            .zip(st.iter())
            .map(|(&pos, &st)| Vertex {
                pos,
                st,
                norm: normal,
            })
            .collect();

        Ok(Face::new(&vs, contact, layer)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse::load_yaml;
    use glm::{vec3, vec2};

    #[test]
    fn extend_vertexes() {
        let mut vertexes = Vec::new();
        let mut indexes = Vec::new();

        let v = Vertex {
            pos: vec3(0., 0., 0.),
            st: vec2(0., 0.),
            norm: vec3(0., 1., 0.),
        };

        let t = FaceVertexes::Triangle([v, v, v]);
        let s = FaceVertexes::Square([v, v, v, v]);

        t.extend_vertexes(&mut vertexes, &mut indexes);

        assert_eq!(vertexes.len(), 3);
        assert_eq!(indexes.len(), 3);
        assert_eq!(indexes[0..=2], [0, 1, 2]);

        s.extend_vertexes(&mut vertexes, &mut indexes);

        assert_eq!(vertexes.len(), 7);
        assert_eq!(indexes.len(), 9);
        assert_eq!(indexes[3..=8], [3, 4, 5, 3, 5, 6]);
    }

    #[test]
    fn parse() {
        let code = r#"
        positions:
          - [ -0.5, 0.5, -0.5 ]
          - [ -0.5, 0.5, 0.5 ]
          - [ 0.5, 0.5, 0.5 ]
          - [ 0.5, 0.5, -0.5 ]

        st:
          - [ 0, 1 ]
          - [ 0, 0 ]
          - [ 1, 0 ]
          - [ 1, 1 ]

        normal: [ 0, 1, 0 ]
        layer: 12
        contact: .
        "#;

        let a: Face = load_yaml(code).unwrap();

        let b = Face {
            vertexes: FaceVertexes::Square([
                Vertex {
                    pos: vec3(-0.5, 0.5, -0.5),
                    st: vec2(0., 1.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(-0.5, 0.5, 0.5),
                    st: vec2(0., 0.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(0.5, 0.5, 0.5),
                    st: vec2(1., 0.),
                    norm: vec3(0., 1., 0.),
                },
                Vertex {
                    pos: vec3(0.5, 0.5, -0.5),
                    st: vec2(1., 1.),
                    norm: vec3(0., 1., 0.),
                },
            ]),
            contact: Sides::all(),
            layer: 12,
        };

        assert_eq!(a, b);
    }
}
