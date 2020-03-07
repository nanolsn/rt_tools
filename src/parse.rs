#[derive(Debug)]
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

#[derive(Debug)]
pub enum VecError {
    Vec3,
    Vec2,
}

impl Parse for glm::Vec3 {
    type DataError = VecError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let v = yml.as_vec().ok_or(VecError::Vec3)?;

        if v.len() != 3 {
            Err(VecError::Vec3)?
        }

        Ok(glm::Vec3::from_iterator(
            v
                .iter()
                .map(|y| f32::parse(y).unwrap())
        ))
    }
}

impl Parse for glm::Vec2 {
    type DataError = VecError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let v = yml.as_vec().ok_or(VecError::Vec2)?;

        if v.len() != 2 {
            Err(VecError::Vec2)?
        }

        Ok(glm::Vec2::from_iterator(
            v
                .iter()
                .map(|y| f32::parse(y).unwrap())
        ))
    }
}

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
            .map(|y| T::parse(y).map_err(|err| Some(err)))
            .collect()
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

impl Parse for u32 {
    type DataError = ();

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        Ok(match yml {
            yaml::Yaml::Integer(n) => *n as u32,
            _ => 0,
        })
    }
}

use super::{
    face::*,
    vertex::Vertex,
    sides::Sides,
};

impl Parse for Face {
    type DataError = FaceError;

    fn parse(yml: &yaml::Yaml) -> Result<Self, Self::DataError> {
        let layer: u32 = Parse::parse(&yml["layer"]).unwrap();
        let contact: Sides = Parse::parse(&yml["contact"]).unwrap();

        let norm: glm::Vec3 = Parse::parse(&yml["norm"])?;
        let pos: Vec<glm::Vec3> = Parse::parse(&yml["pos"])?;
        let st: Vec<glm::Vec2> = Parse::parse(&yml["st"])?;

        let vs: Vec<Vertex> = pos
            .into_iter()
            .zip(st)
            .map(|(pos, st)| Vertex {
                pos,
                st,
                norm,
            })
            .collect();

        Ok(Face::new(&vs, contact, layer)?)
    }
}
