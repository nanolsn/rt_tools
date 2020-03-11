use std::convert::{TryFrom, TryInto};

use super::{
    sides::Sides,
    vertex::Vertex,
    parse::{ParseYaml, VecError, face::yaml_to_face},
};

#[derive(Debug)]
pub enum FaceError {
    IncorrectVertexNumber,
    VecError(VecError),
    ArrayError,
}

impl From<VecError> for FaceError {
    fn from(err: VecError) -> Self { FaceError::VecError(err) }
}

impl From<Option<VecError>> for FaceError {
    fn from(err: Option<VecError>) -> Self {
        match err {
            None => FaceError::ArrayError,
            Some(ve) => FaceError::VecError(ve),
        }
    }
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

impl ParseYaml for Face {
    type DataError = FaceError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> { yaml_to_face(yml) }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
