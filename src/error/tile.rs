#[derive(Debug, Eq, PartialEq)]
pub enum TileField {
    Models,
    Textures,
    States,
}

impl TileField {
    pub fn path(&self) -> &str {
        match self {
            TileField::Models => "models",
            TileField::Textures => "textures",
            TileField::States => "states",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StateError<M, T> {
    ModelError(M),
    TextureError(T),
    TransformError,
    NoLayerDefined,
    NoModelDefined,
    OutOfRange(TileField, usize),
}

impl<M, T> super::Error for StateError<M, T> {
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
pub enum TileError<M, T> {
    StateError(StateError<M, T>),
    NoStates,
}

impl<M, T> From<StateError<M, T>> for TileError<M, T> {
    fn from(err: StateError<M, T>) -> Self { TileError::StateError(err) }
}

impl<M, T> super::Error for TileError<M, T> {
    fn title() -> &'static str { "Tile Error" }

    fn case(&self) -> &str {
        match self {
            TileError::StateError(s) => s.case(),
            TileError::NoStates => "No States",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            TileError::StateError(s) => s.clarification(),
            TileError::NoStates => None,
        }
    }
}

impl<M, T> std::fmt::Display for TileError<M, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", super::Error::display(self))
    }
}
