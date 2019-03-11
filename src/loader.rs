
use std::io::{BufRead, Seek};

pub mod gif;
pub mod png;

use crate::errors::ImageError::InvalidSignature;
use crate::errors::ImageResult;
use crate::types::ImageMeta;



pub fn load<R: BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    match gif::load(image) {
        Ok(meta) => return Ok(meta),
        Err(InvalidSignature) => (),
        otherwise => return otherwise,
    }
    png::load(image)
}
