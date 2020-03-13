pub const DATA_PATH: &str = "data";

pub trait LoadDir {
    const DIR: &'static str;
}

pub trait Load {
    type Error;
    type Loader;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized;
}

use std::rc::Rc;

impl<T> LoadDir for Rc<T>
    where
        T: LoadDir,
{
    const DIR: &'static str = T::DIR;
}

impl<T> Load for Rc<T>
    where
        T: Load,
{
    type Error = T::Error;
    type Loader = T::Loader;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized,
    { Ok(Rc::new(T::load(file, loader)?)) }
}

pub fn load_data_with<T, P>(file: P, loader: &mut T::Loader) -> Result<T, T::Error>
    where
        T: Load + LoadDir,
        P: AsRef<std::path::Path>,
{
    let path = std::path::Path::new(DATA_PATH)
        .join(T::DIR)
        .join(file);

    T::load(path, loader)
}

pub fn load_data<T, P>(file: P) -> Result<T, T::Error>
    where
        T: Load<Loader=()> + LoadDir,
        P: AsRef<std::path::Path>,
{ load_data_with(file, &mut ()) }
