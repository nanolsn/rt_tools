pub enum ParseError<E> {
    ScanError(yaml::ScanError),
    FormatError,
    DataError(E),
}

impl<E> From<yaml::ScanError> for ParseError<E> {
    fn from(err: yaml::ScanError) -> Self { ParseError::ScanError(err) }
}

pub trait Parse
    where
        Self: Sized,
{
    type DataError;
    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError>;
}

pub fn load_yaml<T, S>(code: S) -> Result<T, ParseError<T::DataError>>
    where
        T: Parse,
        S: AsRef<str>,
{
    let ls = yaml::YamlLoader::load_from_str(code.as_ref())?;

    if ls.len() != 1 {
        Err(ParseError::FormatError)?
    }

    let yml = ls.into_iter().next().unwrap();
    T::parse(&yml).map_err(|err| ParseError::DataError(err))
}

impl Parse for super::Sides {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(
            yml
                .as_str()
                .unwrap_or_default()
                .into()
        )
    }
}

impl Parse for glm::Vec3 {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let v = yml.as_vec().ok_or(())?;

        if v.len() != 3 {
            Err(())?
        }

        Ok(glm::Vec3::from_iterator(
            v
                .iter()
                .map(|y| y.as_f64().unwrap() as f32)
        ))
    }
}

impl Parse for glm::Vec2 {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let v = yml.as_vec().ok_or(())?;

        if v.len() != 2 {
            Err(())?
        }

        Ok(glm::Vec2::from_iterator(
            v
                .iter()
                .map(|y| y.as_f64().unwrap() as f32)
        ))
    }
}

impl<T> Parse for Vec<T>
    where
        T: Parse,
{
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(
            yml
                .as_vec()
                .ok_or(())?
                .iter()
                .filter_map(|y| Parse::parse(y).ok())
                .collect()
        )
    }
}
