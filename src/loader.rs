use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

pub mod bmp;
pub mod gif;
pub mod jpeg;
pub mod png;
mod riff;
pub mod webp;

use crate::errors::ImageError::InvalidSignature;
use crate::errors::{ImageError, ImageResult};
use crate::types::{Format, ImageMeta};

macro_rules! try_to_load {
    ($image_type:ident, $image:ident) => {
        match $image_type::load($image) {
            Ok(meta) => return Ok(meta),
            Err(InvalidSignature) => {
                $image.seek(SeekFrom::Start(0))?;
            }
            otherwise => return otherwise,
        }
    };
}

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    try_to_load!(jpeg, image);
    try_to_load!(gif, image);
    try_to_load!(png, image);
    try_to_load!(bmp, image);
    try_to_load!(webp, image);
    Err(ImageError::Unsupported)
}

pub fn load_from_buf(buffer: &[u8]) -> ImageResult<ImageMeta> {
    let mut buffer = std::io::Cursor::new(buffer);
    load(&mut buffer)
}

pub fn load_from_file<T: ?Sized + AsRef<Path>>(file: &T) -> ImageResult<ImageMeta> {
    let file = File::open(file.as_ref())?;
    let mut file = BufReader::new(file);
    load(&mut file)
}

pub fn load_with_format<R: ?Sized + BufRead + Seek>(
    image: &mut R,
    format: Format,
) -> ImageResult<ImageMeta> {
    use Format::*;

    match format {
        Bmp => bmp::load(image),
        Gif => gif::load(image),
        Jpeg => jpeg::load(image),
        Png => png::load(image),
        Webp => webp::load(image),
    }
}
