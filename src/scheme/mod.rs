mod model;
mod tile;

pub trait ConvertFrom<T, L>
    where
        Self: Sized,
{
    type Error;

    fn convert(from: T, loader: &mut L) -> Result<Self, Self::Error>;
}
