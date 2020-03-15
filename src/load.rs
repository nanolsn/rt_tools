pub trait Load
    where
        Self: Sized,
{
    type Error;
    type Loader;

    fn load<S>(file: S, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            S: AsRef<str>;
}

impl<T> Load for std::rc::Rc<T>
    where
        T: Load,
{
    type Error = T::Error;
    type Loader = T::Loader;

    fn load<S>(file: S, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            S: AsRef<str>,
    { Ok(std::rc::Rc::new(T::load(file, loader)?)) }
}

impl<T> Load for Box<T>
    where
        T: Load,
{
    type Error = T::Error;
    type Loader = T::Loader;

    fn load<S>(file: S, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            S: AsRef<str>,
    { Ok(Box::new(T::load(file, loader)?)) }
}
