use thiserror::Error;

pub(crate) type MyResult<T = ()> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generic Error: {0}")]
    Generic(String),
}
