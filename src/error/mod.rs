pub mod model;
pub mod tile;

pub trait Error {
    fn title() -> &'static str;

    fn case(&self) -> &str;

    fn clarification(&self) -> Option<String> { None }

    fn advice(&self) -> Option<String> { None }

    fn display(&self) -> ErrorFormatter<Self> { ErrorFormatter(self) }
}

pub struct ErrorFormatter<'a, T: ?Sized>(pub &'a T);

impl<'a, T> std::fmt::Display for ErrorFormatter<'a, T>
    where
        T: Error,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", T::title(), self.0.case())?;

        if let Some(s) = self.0.clarification() {
            write!(f, " ({})", s)?
        }

        if let Some(s) = self.0.advice() {
            write!(f, " [{}]", s)?
        }

        Ok(())
    }
}
