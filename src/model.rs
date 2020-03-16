use super::{
    face::{Face, FaceError},
    sides::Sides,
    vertex::Vertex,
};

#[derive(Debug, Eq, PartialEq)]
pub enum ModelError {
    FacesError,
    FaceError(FaceError),
    ArrayError,
}

impl From<FaceError> for ModelError {
    fn from(err: FaceError) -> Self { ModelError::FaceError(err) }
}

impl super::error::Error for ModelError {
    fn title() -> &'static str { "Model Error" }

    fn case(&self) -> &str {
        match self {
            ModelError::FacesError => "Faces Error",
            ModelError::FaceError(fe) => fe.case(),
            ModelError::ArrayError => "Array Error",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            ModelError::FaceError(fe) => fe.clarification(),
            _ => None,
        }
    }
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", super::error::Error::display(self))
    }
}

#[derive(Debug, PartialEq)]
pub struct Model {
    pub faces: Vec<Face>,
    pub full_sides: Sides,
}

impl Model {
    pub fn get_indexed_vertexes(&self) -> (Vec<Vertex>, Vec<u32>) {
        let min_capacity = self.faces.len() * 3;
        let mut vertexes = Vec::with_capacity(min_capacity);
        let mut indexes = Vec::with_capacity(min_capacity);

        for face in &self.faces {
            face.vertexes.extend_vertexes(&mut vertexes, &mut indexes);
        }

        (vertexes, indexes)
    }
}
