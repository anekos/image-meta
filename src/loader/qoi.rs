use std::io::{BufRead, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::errors::{ImageError, ImageResult};
use crate::types::{Color, Dimensions, Format, ImageMeta};

// See: https://github.com/phoboslab/qoi/blob/master/qoi.h

const SIGNATURE: [u8; 4] = [0x71, 0x6f, 0x69, 0x66];

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;

    let (dimensions, color) = read_header(image)?;

    Ok(ImageMeta {
        animation_frames: None,
        color,
        dimensions,
        format: Format::Qoi,
    })
}

fn read_signature<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult {
    let mut signature = [0u8; 4];
    image.read_exact(&mut signature)?;
    if SIGNATURE != signature {
        return Err(ImageError::InvalidSignature);
    }
    Ok(())
}

fn read_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    use crate::types::ColorMode::*;

    let width = image.read_u32::<BigEndian>()?;
    let height = image.read_u32::<BigEndian>()?;
    let color = image.read_u8()?;
    let (mode, alpha_channel) = match color {
        3 => (Rgb, false),
        4 => (Rgb, true),
        _ => {
            return Err(ImageError::CorruptImage(
                format!("Invalid color type: {}", color).into(),
            ))
        }
    };
    let color = Color {
        mode,
        alpha_channel,
        resolution: 8,
    };

    Ok((Dimensions { height, width }, color))
}
