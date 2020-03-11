use crate::parse::{
    Parse,
    YamlError,
    parse_file,
};

pub const DATA_PATH: &str = "data";

pub trait LoadDir {
    const DIR: &'static str;
}

pub trait Load: LoadDir {
    type Error;
    type Loader;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized;
}

impl<T> Load for T
    where
        T: LoadDir + Parse,
{
    type Error = YamlError<T::DataError>;
    type Loader = ();

    fn load<P>(file: P, _: &mut ()) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    { parse_file(file) }
}

pub fn load_data_with<T, P>(file: P, loader: &mut T::Loader) -> Result<T, T::Error>
    where
        T: Load,
        P: AsRef<std::path::Path>,
{
    let path = std::path::Path::new(DATA_PATH)
        .join(T::DIR)
        .join(file);

    T::load(path, loader)
}

pub fn load_data<T, P>(file: P) -> Result<T, T::Error>
    where
        T: Load<Loader=()>,
        P: AsRef<std::path::Path>,
{ load_data_with(file, &mut ()) }
