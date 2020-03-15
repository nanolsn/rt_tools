use super::{
    shell_transform::Shell,
};

#[derive(Debug)]
pub enum StateError<M, T> {
    ModelError(M),
    TextureError(T),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
    OutOfRange,
}

#[derive(Debug)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
