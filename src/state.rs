use super::{
    shell_transform::Shell,
    tile::TileField,
};

#[derive(Debug, PartialEq)]
pub enum StateError<M, T> {
    ModelError(M),
    TextureError(T),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
    OutOfRange(TileField, usize),
}

impl<M, T> super::error::Error for StateError<M, T> {
    fn title() -> &'static str { "State Error" }

    fn case(&self) -> &str {
        match self {
            StateError::ModelError(_) => "Model Error",
            StateError::TextureError(_) => "Texture Error",
            StateError::TransformError => "Transform Error",
            StateError::NoLayerDefined => "No Layer Defined",
            StateError::NoModelDefined => "NoModel Defined",
            StateError::OutOfRange(_, _) => "Out Of Range",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            StateError::OutOfRange(f, i) => Some(format!("at {}[{}]", f.path(), i)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
