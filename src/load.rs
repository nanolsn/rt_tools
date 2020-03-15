pub trait Load {
    type Error;
    type Loader;

    fn load<P>(file: P, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            P: AsRef<std::path::Path>,
            Self: Sized;
}

use std::rc::Rc;
use super::asset::Asset;

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

impl<T> Asset for Rc<T>
    where
        T: Asset,
{
    const DIR: &'static str = T::DIR;
}
