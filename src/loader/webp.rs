

use std::io::{BufRead, Read, Seek};

use crate::errors::{ImageError, ImageResult};
use crate::types::{Color, Dimensions, Format, ImageMeta};
use crate::loader::riff::{Chunk, RiffReader};



const WEBP: [u8; 4] = *b"WEBP";
const VP8: [u8; 4] = *b"VP8 ";
const VP8L: [u8; 4] = *b"VP8L";
const KEY_FRAME: [u8;3] = [0x9d, 0x01, 0x2a];


pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    let mut reader = RiffReader::open(image)?;

    if *reader.form_type() != WEBP {
        return Err(ImageError::InvalidSignature);
    }

    while let Some(mut chunk) = reader.read_chunk()? {
        let dimensions = match *chunk.identifier() {
            VP8 => read_vp8_chunk(&mut chunk)?,
            VP8L => continue,
            _ => continue,
        };
        return Ok(ImageMeta {
            animation_frames: None, // FIXME
            color: Color::RgbA(8), // FIXME
            dimensions,
            format: Format::Webp,
        })
    }

    Err(ImageError::CorruptImage("Expected chunk not found".into()))
}


fn read_vp8_chunk(chunk: &mut Chunk) -> ImageResult<Dimensions> {
    // See https://tools.ietf.org/html/rfc6386#page-30

    let mut bits = [0u8;3];
    chunk.read_exact(&mut bits)?;
    let key_frame = 0 == bits[0] & 1;

    if key_frame {
        let mut code = [0u8;3];
        chunk.read_exact(&mut code)?;
        if code != KEY_FRAME {
            return Err(ImageError::CorruptImage(format!("Invalid key frame code: {:?}", code).into()));
        }

        let mut bits = [0u8;2];
        chunk.read_exact(&mut bits)?;
        let (width, _) = extract_dimension(bits);
        chunk.read_exact(&mut bits)?;
        let (height, _) = extract_dimension(bits);

        return Ok(Dimensions {
            width: u32::from(width),
            height: u32::from(height),
        });
    }

    Err(ImageError::CorruptImage("Not key frame".into()))
}

fn extract_dimension(bits: [u8;2]) -> (u16, u8) {
    let size = u16::from(bits[1] & 0b0011_1111) << 8 | u16::from(bits[0]);
    let scale = (bits[1] & 0b1100_0000) >> 6;
    (size, scale)
}
