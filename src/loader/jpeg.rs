use std::io::{BufRead, Cursor, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::errors::{ImageError, ImageResult};
use crate::types::{Color, ColorMode, Dimensions, Format, ImageMeta};

const MARKER: u8 = 0xff;
const SOI: u8 = 0xd8;

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;
    let dimensions = read_sof(image)?;
    let color = Color {
        alpha_channel: false,
        mode: ColorMode::Rgb,
        resolution: 8,
    };
    Ok(ImageMeta {
        animation_frames: None,
        color,
        dimensions,
        format: Format::Jpeg,
    })
}

fn read_signature<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult {
    let mut soi = [0u8; 2];
    image.read_exact(&mut soi)?;
    if [MARKER, SOI] != soi {
        return Err(ImageError::InvalidSignature);
    }
    Ok(())
}

fn read_sof<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<Dimensions> {
    loop {
        if let (_, Some(data)) = read_segment(image, is_sof)? {
            let mut data = Cursor::new(data);
            data.seek(SeekFrom::Current(1))?;
            let height = data.read_u16::<BigEndian>().map(u32::from)?;
            let width = data.read_u16::<BigEndian>().map(u32::from)?;
            return Ok(Dimensions { width, height });
        }
    }
}

fn read_segment<R: ?Sized + BufRead + Seek, F>(
    image: &mut R,
    target_marker: F,
) -> ImageResult<(u8, Option<Vec<u8>>)>
where
    F: Fn(u8) -> bool,
{
    let prefix = image.read_u8()?;
    if prefix != MARKER {
        return Err(ImageError::CorruptImage("Marker not found".into()));
    }

    // Skip stuffing bytes
    let mut marker = image.read_u8()?;
    while marker == MARKER {
        marker = image.read_u8()?;
    }

    let length = image.read_u16::<BigEndian>()? - 2;

    if target_marker(marker) {
        let mut result = vec![0u8; length as usize];
        image.read_exact(&mut result)?;
        Ok((marker, Some(result)))
    } else {
        image.seek(SeekFrom::Current(i64::from(length)))?;
        Ok((marker, None))
    }
}

fn is_sof(marker: u8) -> bool {
    matches!(
        marker,
        0xc0 | 0xc1 | 0xc2 | 0xc3 | 0xc5 | 0xc6 | 0xc7 | 0xc9 | 0xca | 0xcb | 0xcd | 0xce | 0xcf
    )
}
