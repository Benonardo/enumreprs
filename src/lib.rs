use thiserror::Error;

pub trait IntoRepr<T> {
    fn into_repr(self) -> T;
}

pub trait FromRepr<T> {
    fn from_repr(repr: T) -> Result<Self, FromReprError<T>>
    where
        Self: Sized;
}

#[derive(Error, Debug, PartialEq)]
pub enum FromReprError<T> {
    #[error("Invalid variant `{0}`")]
    InvalidVariant(T),
}
