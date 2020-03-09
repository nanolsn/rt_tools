use super::{
    model::Model,
    shell_transform::Shell,
};

#[derive(Debug)]
pub enum StateError {
    NoLayerDefined,
    NoModelDefined,
}

#[derive(Debug)]
pub struct State<'m> {
    pub model: &'m Model,
    pub shell: Shell,
    pub layers: Vec<u32>,
    pub id: u32,
}
