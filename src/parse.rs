use super::scheme::ConvertFrom;

#[derive(Debug)]
pub enum ParseError<E> {
    YamlError(serde_yaml::Error),
    ConvertError(E),
}

impl<E> From<serde_yaml::Error> for ParseError<E> {
    fn from(err: serde_yaml::Error) -> Self { ParseError::YamlError(err) }
}

pub fn parse_with<S, T, I, L>(code: S, loader: L) -> Result<T, ParseError<T::Error>>
    where
        S: AsRef<str>,
        T: ConvertFrom<I, L>,
        I: serde::de::DeserializeOwned,
{
    let from: I = serde_yaml::from_str(code.as_ref())?;
    let item = T::convert(from, loader)
        .map_err(|e| ParseError::ConvertError(e))?;

    Ok(item)
}

pub fn parse<S, T, I>(code: S) -> Result<T, ParseError<T::Error>>
    where
        S: AsRef<str>,
        T: ConvertFrom<I, ()>,
        I: serde::de::DeserializeOwned,
{ parse_with(code, ()) }
