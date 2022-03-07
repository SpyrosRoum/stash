use std::borrow::Cow;

pub type StashResult<T> = std::result::Result<T, StashError>;

#[derive(Debug, thiserror::Error)]
pub enum StashError {
    #[error("{0}")]
    BadValue(Cow<'static, str>),
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
}
