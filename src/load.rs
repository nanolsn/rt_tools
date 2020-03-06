pub const DATA_PATH: &str = "data";

pub trait Load {
    const DIR: &'static str;

    fn load<P>(file: P) -> Self
        where
            P: AsRef<std::path::Path>;
}

pub fn load_data<T, P>(file: P) -> T
    where
        T: Load,
        P: AsRef<std::path::Path>,
{
    let path = std::path::Path::new(DATA_PATH)
        .join(T::DIR)
        .join(file);

    T::load(path)
}
