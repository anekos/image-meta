
use std::borrow::Cow;
use thiserror::Error;



pub type ImageResult<T> = Result<T, ImageError>;
pub type ImageResultU = Result<(), ImageError>;



#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Corrupt image: {0}")]
    CorruptImage(Cow<'static, str>),
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unsupported format")]
    Unsupported,
}
