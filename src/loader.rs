
use std::io::{BufRead, Seek, SeekFrom};

pub mod gif;
pub mod jpeg;
pub mod png;

use crate::errors::ImageError::InvalidSignature;
use crate::errors::ImageResult;
use crate::types::ImageMeta;



macro_rules! try_to_load {
    ($image_type:ident, $image:ident) => {
        match $image_type::load($image) {
            Ok(meta) => return Ok(meta),
            Err(InvalidSignature) => {
                $image.seek(SeekFrom::Start(0))?;
            },
            otherwise => return otherwise,
        }
    }
}

pub fn load<R: BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    try_to_load!(gif, image);
    try_to_load!(jpeg, image);
    png::load(image)
}
