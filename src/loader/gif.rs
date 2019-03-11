
use std::io::{BufRead, Seek};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::errors::{ImageError, ImageResult, ImageResultU};
use crate::types::{Dimensions, Format, ImageMeta};



const GIF87A: [u8; 6] = *b"GIF87a";
const GIF89A: [u8; 6] = *b"GIF89a";


pub fn load<R: BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;
    let dimensions = read_header(image)?;
    Ok(ImageMeta {
        dimensions,
        format: Format::Gif,
    })
}

fn read_signature<R: BufRead + Seek>(image: &mut R) -> ImageResultU {
    let mut signature = [0u8;6];
    image.read_exact(&mut signature)?;
    match signature {
        GIF87A | GIF89A => Ok(()),
        _ => Err(ImageError::InvalidSignature),
    }
}

fn read_header<R: BufRead + Seek>(image: &mut R) -> ImageResult<Dimensions> {
    let width = image.read_u16::<LittleEndian>().map(u32::from)?;
    let height = image.read_u16::<LittleEndian>().map(u32::from)?;
    Ok(Dimensions { width, height })
}
