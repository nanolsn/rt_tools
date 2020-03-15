use super::{
    shell_transform::Shell,
};

#[derive(Debug, PartialEq)]
pub enum StateError<M, T> {
    ModelError(M),
    TextureError(T),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
    OutOfRange,
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
