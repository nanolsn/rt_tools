use super::{
    face::{Face, FaceError},
    sides::Sides,
    vertex::Vertex,
    parse::{ParseYaml, model::yaml_to_model, parse_file, YamlError, VecError},
    load::Load,
};

#[derive(Debug)]
pub enum ModelError {
    FacesError,
    FaceError(FaceError),
    VecError(VecError),
    ArrayError,
}

impl From<FaceError> for ModelError {
    fn from(err: FaceError) -> Self { ModelError::FaceError(err) }
}

impl From<VecError> for ModelError {
    fn from(err: VecError) -> Self { ModelError::VecError(err) }
}

impl From<Option<VecError>> for ModelError {
    fn from(err: Option<VecError>) -> Self {
        match err {
            None => ModelError::ArrayError,
            Some(ve) => ModelError::VecError(ve),
        }
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

impl ParseYaml for Model {
    type DataError = ModelError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> { yaml_to_model(yml) }
}

impl Load for Model {
    const DIR: &'static str = "models";

    type Error = YamlError<ModelError>;
    type Loader = ();

    fn load<P>(file: P, _: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    { parse_file(file) }
}
