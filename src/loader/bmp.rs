
use std::io::{BufRead, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::errors::{ImageError, ImageResult, ImageResultU};
use crate::types::{Color, Dimensions, Format, ImageMeta};



const BM: [u8; 2] = *b"BM";


pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;
    let (dimensions, color) = read_header(image)?;

    Ok(ImageMeta {
        animation_frames: None,
        color,
        dimensions,
        format: Format::Bmp,
    })
}

fn read_signature<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResultU {
    let mut signature = [0u8;2];
    image.read_exact(&mut signature)?;
    if signature != BM {
        return Err(ImageError::InvalidSignature);
    }
    // Skip rest file header
    image.seek(SeekFrom::Current(12))?;
    Ok(())
}

fn read_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    let header_size = image.read_u32::<LittleEndian>()?;
    match header_size {
        12 | 64 => read_os2_header(image),
        40 | 108 | 124 => read_windows_header(image),
        sz => Err(ImageError::CorruptImage(format!("Unsupported header size: {}", sz).into()))
    }
}

fn read_windows_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    let width = image.read_u32::<LittleEndian>()?;
    let height = image.read_i32::<LittleEndian>()?.abs() as u32;
    image.seek(SeekFrom::Current(2))?; // planes

    let resolution = image.read_u16::<LittleEndian>()? / 3;
    let dimensions = Dimensions { height, width };
    let color = Color::Rgb(resolution as u8);

    Ok((dimensions, color))
}

fn read_os2_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    let width = image.read_u16::<LittleEndian>().map(u32::from)?;
    let height = image.read_i16::<LittleEndian>()?.abs() as u32;
    image.seek(SeekFrom::Current(2))?; // planes

    let resolution = image.read_u16::<LittleEndian>()? / 3;

    let dimensions = Dimensions { height, width };
    let color = Color::Rgb(resolution as u8);

    Ok((dimensions, color))
}
