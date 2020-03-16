#[derive(Debug, Eq, PartialEq)]
pub enum ModelField {
    Pos,
    St,
    Norm,
    DataPos,
    DataSt,
    DataNorm,
}

impl ModelField {
    pub fn path(&self) -> &str {
        match self {
            ModelField::Pos => "pos",
            ModelField::St => "st",
            ModelField::Norm => "norm",
            ModelField::DataPos => "data.pos",
            ModelField::DataSt => "data.st",
            ModelField::DataNorm => "data.norm",
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub enum FaceError {
    WrongVertexNumber(ModelField),
    ArrayError,
    OutOfRange(ModelField, usize),
    IncorrectDataFormat,
}

impl super::Error for FaceError {
    fn title() -> &'static str { "Face Error" }

    fn case(&self) -> &str {
        match self {
            FaceError::WrongVertexNumber(_) => "Wrong Vertex Number",
            FaceError::ArrayError => "Array Error",
            FaceError::OutOfRange(..) => "Out of Range",
            FaceError::IncorrectDataFormat => "Incorrect Data Format",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            FaceError::WrongVertexNumber(f) => Some(format!("at {}", f.path())),
            FaceError::OutOfRange(f, i) => Some(format!("at {}[{}]", f.path(), i)),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ModelError {
    FacesError,
    FaceError(FaceError),
    ArrayError,
}

impl From<FaceError> for ModelError {
    fn from(err: FaceError) -> Self { ModelError::FaceError(err) }
}

impl super::Error for ModelError {
    fn title() -> &'static str { "Model Error" }

    fn case(&self) -> &str {
        match self {
            ModelError::FacesError => "Faces Error",
            ModelError::FaceError(fe) => fe.case(),
            ModelError::ArrayError => "Array Error",
        }
    }

    fn clarification(&self) -> Option<String> {
        match self {
            ModelError::FaceError(fe) => fe.clarification(),
            _ => None,
        }
    }
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", super::Error::display(self))
    }
}
