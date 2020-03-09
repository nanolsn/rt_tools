use super::{
    model::ModelError,
    shell_transform::Shell,
    parse::ParseError,
};

#[derive(Debug)]
pub enum StateError {
    ModelParseError(ParseError<ModelError>),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
}

impl From<ParseError<ModelError>> for StateError {
    fn from(err: ParseError<ModelError>) -> Self { StateError::ModelParseError(err) }
}

#[derive(Debug)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
