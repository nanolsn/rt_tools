use super::shell_transform::Shell;

#[derive(Debug, PartialEq)]
pub struct State {
    pub model: usize,
    pub shell: Shell,
    pub layers: Vec<u32>,
}
