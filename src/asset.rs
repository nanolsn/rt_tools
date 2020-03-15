pub const DATA_PATH: &str = "assets";

pub trait Asset: super::load::Load {
    const DIR: &'static str;

    fn full_path<S>(file: S) -> String
        where
            S: AsRef<str>,
    {
        let mut s = String::from(DATA_PATH);
        s.push(std::path::MAIN_SEPARATOR);
        s.push_str(Self::DIR);
        s.push(std::path::MAIN_SEPARATOR);
        s.push_str(file.as_ref());
        s
    }

    fn load_asset<S>(file: S, loader: &mut Self::Loader) -> Result<Self, Self::Error>
        where
            S: AsRef<str>,
    { Self::load(Self::full_path(file), loader) }
}

impl<T: Asset> Asset for std::rc::Rc<T> { const DIR: &'static str = T::DIR; }

impl<T: Asset> Asset for Box<T> { const DIR: &'static str = T::DIR; }
