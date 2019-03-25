
use std::io::{BufRead, Seek, SeekFrom};

use byteorder::{ReadBytesExt, LittleEndian};

use crate::errors::{ImageError, ImageResult, ImageResultU};
use crate::types::{Color, Dimensions, Format, ImageMeta};




#[derive(Default)]
struct BlockReader {
    frames: usize,
}


pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;
    let (dimensions, color) = read_header(image)?;

    let mut reader = BlockReader::default();
    reader.read(image)?;

    Ok(ImageMeta {
        animation_frames: if 1 < reader.frames { Some(reader.frames) } else { None },
        color,
        dimensions,
        format: Format::Gif,
    })
}

fn read_signature<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResultU {
    let mut signature = [0u8;6];
    image.read_exact(&mut signature)?;
    match &signature {
        b"GIF87a" | b"GIF89a" => Ok(()),
        _ => Err(ImageError::InvalidSignature),
    }
}

fn read_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    let width = image.read_u16::<LittleEndian>().map(u32::from)?;
    let height = image.read_u16::<LittleEndian>().map(u32::from)?;

    let bits = image.read_u8()?;
    let table_bytes = read_table_bits(bits)?;
    let resolution = (bits & 0b0111_0000) >> 4;

    // 1 Background color index
    // 1 Aspect Ratio

    image.seek(SeekFrom::Current(table_bytes + 2))?;

    Ok((Dimensions { width, height }, Color::Palette(resolution + 1)))
}

impl BlockReader {
    fn read<R: ?Sized + BufRead + Seek>(&mut self, image: &mut R) -> ImageResultU {
        loop {
            let b = image.read_u8()?;
            match b {
                0x21 => self.read_extension(image)?,
                0x2c => self.read_image_data(image)?,
                0x3b => return Ok(()),
                x => return Err(ImageError::CorruptImage(format!("Unknown block: {:x}", x).into())),
            };
        }
    }

    fn read_extension<R: ?Sized + BufRead + Seek>(&mut self, image: &mut R) -> ImageResultU {
        match image.read_u8()? {
            0x01 | 0xf9 | 0xfe | 0xff => (),
            x => return Err(ImageError::CorruptImage(format!("Unknown extension: {:x}", x).into())),
        };
        loop {
            let size = image.read_u8()?;
            if size == 0 {
                return Ok(());
            }
            image.seek(SeekFrom::Current(i64::from(size)))?;
        }
    }

    fn read_image_data<R: ?Sized + BufRead + Seek>(&mut self, image: &mut R) -> ImageResultU {
        // 2 Left
        // 2 Top
        // 2 Width
        // 2 Height
        image.seek(SeekFrom::Current(8))?;

        let table_bytes = read_table_bits(image.read_u8()?)?;
        image.seek(SeekFrom::Current(table_bytes + 1))?; // `+ 1` means LZW minimum code size

        loop {
            let size = image.read_u8()?;
            if size == 0 {
                break;
            }
            image.seek(SeekFrom::Current(i64::from(size)))?;
        }

        self.frames += 1;
        Ok(())
    }
}

/// Returns the bytes to skip
fn read_table_bits(bits: u8) -> ImageResult<i64> {
    let has_table = (bits & 0b1000_0000) > 0;
    let table_size = 2 << (bits & 0b0000_0111);
    if has_table {
        Ok(i64::from(table_size) * 3)
    } else {
        Ok(0)
    }
}
