
use failure::Fail;



pub type ImageResult<T> = Result<T, ImageError>;
pub type ImageResultU = Result<(), ImageError>;



#[derive(Fail, Debug)]
pub enum ImageError {
    #[fail(display = "Invalid signature")]
    InvalidSignature,
    #[fail(display = "Corrupt image: {}", 0)]
    CorruptImage(&'static str),
    #[fail(display = "IO Error: {}", 0)]
    Io(std::io::Error),
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
