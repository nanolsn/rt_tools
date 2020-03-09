use super::{
    face::{Face, FaceError},
    sides::Sides,
    vertex::Vertex,
    parse::{Parse, model::yaml_to_model},
};

#[derive(Debug)]
pub enum ModelError {
    FacesError,
    FaceError(FaceError),
}

impl From<FaceError> for ModelError {
    fn from(err: FaceError) -> Self { ModelError::FaceError(err) }
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

impl Parse for Model {
    type DataError = ModelError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> { yaml_to_model(yml) }
}
