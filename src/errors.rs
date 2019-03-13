
use std::borrow::Cow;
use failure::Fail;



pub type ImageResult<T> = Result<T, ImageError>;
pub type ImageResultU = Result<(), ImageError>;



#[derive(Fail, Debug)]
pub enum ImageError {
    #[fail(display = "Corrupt image: {}", 0)]
    CorruptImage(Cow<'static, str>),
    #[fail(display = "Invalid signature")]
    InvalidSignature,
    #[fail(display = "IO Error: {}", 0)]
    Io(std::io::Error),
    #[fail(display = "Unsupported format")]
    Unsupported,
}


macro_rules! define_error {
    ($source:ty, $kind:ident) => {
        impl From<$source> for ImageError {
            fn from(error: $source) -> ImageError {
                ImageError::$kind(error)
            }
        }
    }
}

define_error!(std::io::Error, Io);
