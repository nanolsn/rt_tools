pub mod face;
pub mod model;
pub mod point;
pub mod sides;
pub mod state;
pub mod tile;

#[derive(Debug)]
pub enum ParseError<E> {
    IOError(std::io::Error),
    ScanError(yaml::ScanError),
    FormatError,
    DataError(E),
}

impl<E> From<std::io::Error> for ParseError<E> {
    fn from(err: std::io::Error) -> Self { ParseError::IOError(err) }
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

pub fn parse<T>(yml: &yaml::Yaml) -> Result<T, T::DataError>
    where
        T: Parse,
{ T::parse(yml) }

pub fn parse_default<T>(yml: &yaml::Yaml) -> T
    where
        T: Parse + Default,
{ parse(yml).unwrap_or_default() }

pub fn parse_code<T, S>(code: S) -> Result<T, ParseError<T::DataError>>
    where
        T: Parse,
        S: AsRef<str>,
{
    let ls = yaml::YamlLoader::load_from_str(code.as_ref())?;

    if ls.len() != 1 { Err(ParseError::FormatError)? }

    let yml = ls.into_iter().next().unwrap();
    parse(&yml).map_err(|err| ParseError::DataError(err))
}

pub fn parse_file<T, P>(path: P) -> Result<T, ParseError<T::DataError>>
    where
        T: Parse,
        P: AsRef<std::path::Path>,
{
    let code = std::fs::read_to_string(path)?;
    parse_code(code)
}

#[derive(Debug)]
pub enum VecError {
    Vec4,
    Vec3,
    Vec2,
    Vec1,
}

macro_rules! impl_vec {
    ($($i:ident $n:literal),*) => {
        $(impl Parse for glm::$i {
            type DataError = VecError;

            fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
                let v = yml.as_vec().ok_or(VecError::$i)?;

                if v.len() != $n {
                    Err(VecError::$i)?
                }

                Ok(glm::$i::from_iterator(
                    v
                        .iter()
                        .map(|y| f32::parse(y).unwrap())
                ))
            }
        })*
    };
}
impl_vec!(Vec4 4, Vec3 3, Vec2 2, Vec1 1);

impl<T> Parse for Vec<T>
    where
        T: Parse,
{
    type DataError = Option<T::DataError>;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        yml
            .as_vec()
            .ok_or(None)?
            .iter()
            .map(|y| parse(y).map_err(|err| Some(err)))
            .collect()
    }
}

impl<T> Parse for Option<T>
    where
        T: Parse,
{
    type DataError = T::DataError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(if yml.is_badvalue() {
            None
        } else {
            Some(parse(yml)?)
        })
    }
}

impl Parse for f32 {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(match yml {
            r @ yaml::Yaml::Real(_) => r.as_f64().unwrap() as f32,
            yaml::Yaml::Integer(n) => *n as f32,
            _ => 0.,
        })
    }
}

impl Parse for f64 {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(match yml {
            r @ yaml::Yaml::Real(_) => r.as_f64().unwrap(),
            yaml::Yaml::Integer(n) => *n as f64,
            _ => 0.,
        })
    }
}

macro_rules! impl_int {
    ($($t:ty)*) => {
        $(impl Parse for $t {
            type DataError = ();

            fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
                Ok(match yml {
                    yaml::Yaml::Integer(n) => *n as $t,
                    _ => 0,
                })
            }
        })*
    };
}
impl_int!(u8 i8 u16 i16 u32 i32 u64 i64 isize usize);

impl Parse for String {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(yml
            .as_str()
            .ok_or(())?
            .into()
        )
    }
}
