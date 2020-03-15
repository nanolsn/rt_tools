use super::load::Load;

pub const DATA_PATH: &str = "assets";

pub trait Asset: Load {
    const DIR: &'static str;
}

pub fn load_asset_with<T, P>(file: P, loader: &mut T::Loader) -> Result<T, T::Error>
    where
        T: Asset,
        P: AsRef<std::path::Path>,
{
    let path = std::path::Path::new(DATA_PATH)
        .join(T::DIR)
        .join(file);

    T::load(path, loader)
}

pub fn load_asset<T, P>(file: P) -> Result<T, T::Error>
    where
        T: Asset<Loader=()>,
        P: AsRef<std::path::Path>,
{ load_asset_with(file, &mut ()) }
