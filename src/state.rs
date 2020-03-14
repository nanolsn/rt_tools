use super::{
    model::ModelError,
    shell_transform::Shell,
    parse::YamlError,
};

#[derive(Debug)]
pub enum StateError {
    ModelParseError(YamlError<ModelError>),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
    OutOfRange,
}

impl From<YamlError<ModelError>> for StateError {
    fn from(err: YamlError<ModelError>) -> Self { StateError::ModelParseError(err) }
}

#[derive(Debug)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
