use crate::parse::{
    Parse,
    ParseError,
    parse_file,
};

pub const DATA_PATH: &str = "data";

pub trait LoadDir {
    const DIR: &'static str;
}

pub trait Load: LoadDir {
    type Error;

    fn load<P>(file: P) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized;
}

impl<T> Load for T
    where
        T: LoadDir + Parse,
{
    type Error = ParseError<T::DataError>;

    fn load<P>(file: P) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    { parse_file(file) }
}

pub fn load_data<T, P>(file: P) -> Result<T, T::Error>
    where
        T: Load,
        P: AsRef<std::path::Path>,
{
    let path = std::path::Path::new(DATA_PATH)
        .join(T::DIR)
        .join(file);

    T::load(path)
}
