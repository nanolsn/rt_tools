use super::{
    sides::Sides,
    vertex::Vertex,
};

#[derive(Debug, Eq, PartialEq)]
pub enum FaceField {
    Pos,
    St,
    Norm,
    DataPos,
    DataSt,
    DataNorm,
}

impl FaceField {
    pub fn path(&self) -> &str {
        match self {
            FaceField::Pos => "pos",
            FaceField::St => "st",
            FaceField::Norm => "norm",
            FaceField::DataPos => "data.pos",
            FaceField::DataSt => "data.st",
            FaceField::DataNorm => "data.norm",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum FaceError {
    WrongVertexNumber(FaceField),
    ArrayError,
    OutOfRange(FaceField, usize),
    IncorrectDataFormat,
}

impl super::error::Error for FaceError {
    fn title() -> &'static str { "Face Error" }

    fn case(&self) -> &str {
        match self {
            FaceError::WrongVertexNumber(_) => "Wrong Vertex Number",
            FaceError::ArrayError => "Array Error",
            FaceError::OutOfRange(..) => "Out of Range",
            FaceError::IncorrectDataFormat => "Incorrect Data Format",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            FaceError::WrongVertexNumber(f) => Some(format!("at {}", f.path())),
            FaceError::OutOfRange(f, i) => Some(format!("at {}[{}]", f.path(), i)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FaceVertexes {
    Triangle([Vertex; 3]),
    Square([Vertex; 4]),
}

impl FaceVertexes {
    pub fn from_slice(vs: &[Vertex]) -> Option<Self> {
        match vs {
            &[x, y, z] => Some(FaceVertexes::Triangle([x, y, z])),
            &[x, y, z, w] => Some(FaceVertexes::Square([x, y, z, w])),
            _ => None,
        }
    }
}

impl From<[Vertex; 3]> for FaceVertexes {
    fn from(vs: [Vertex; 3]) -> Self { FaceVertexes::Triangle(vs) }
}

impl From<[Vertex; 4]> for FaceVertexes {
    fn from(vs: [Vertex; 4]) -> Self { FaceVertexes::Square(vs) }
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
