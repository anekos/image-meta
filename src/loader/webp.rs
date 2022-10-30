use std::io::{BufRead, Read, Seek};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::errors::{ImageError, ImageResult};
use crate::loader::riff::{Chunk, RiffReader};
use crate::types::{Color, ColorMode, Dimensions, Format, ImageMeta};

pub struct WebpReader<T: BufRead + Seek> {
    animation_frames: usize,
    dimensions: Option<Dimensions>,
    riff: RiffReader<T>,
}

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    let mut reader = WebpReader::new(RiffReader::open(image)?);
    reader.read()?;

    let dimensions = reader
        .dimensions
        .ok_or_else(|| ImageError::CorruptImage("VP8? chunk not found".into()))?;
    let animation_frames = if 0 < reader.animation_frames {
        Some(reader.animation_frames)
    } else {
        None
    };
    let color = Color {
        alpha_channel: true,
        mode: ColorMode::Rgb,
        resolution: 8,
    };

    Ok(ImageMeta {
        animation_frames,
        color,
        dimensions,
        format: Format::Webp,
    })
}

impl<T: BufRead + Seek> WebpReader<T> {
    pub fn new(riff: RiffReader<T>) -> Self {
        Self {
            animation_frames: 0,
            dimensions: None,
            riff,
        }
    }

    fn read(&mut self) -> ImageResult {
        if self.riff.form_type() != b"WEBP" {
            return Err(ImageError::InvalidSignature);
        }

        while let Some(mut chunk) = self.riff.read_chunk()? {
            match chunk.identifier() {
                b"ANMF" => self.animation_frames += 1,
                b"VP8 " => self.dimensions = Some(read_vp8_chunk(&mut chunk)?),
                b"VP8L" => self.dimensions = Some(read_vp8l_chunk(&mut chunk)?),
                b"VP8X" => self.dimensions = Some(read_vp8x_chunk(&mut chunk)?),
                _ => (),
            }
        }
        Ok(())
    }
}

fn read_vp8_chunk(chunk: &mut Chunk) -> ImageResult<Dimensions> {
    // See https://tools.ietf.org/html/rfc6386#page-30

    let mut bits = [0u8; 3];
    chunk.read_exact(&mut bits)?;
    let key_frame = 0 == bits[0] & 1;

    if key_frame {
        let mut signature = [0u8; 3];
        chunk.read_exact(&mut signature)?;
        if signature != [0x9d, 0x01, 0x2a] {
            return Err(ImageError::CorruptImage(
                format!("Invalid key frame code: {:?}", signature).into(),
            ));
        }

        let mut bits = [0u8; 2];
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

fn read_vp8l_chunk(chunk: &mut Chunk) -> ImageResult<Dimensions> {
    // See https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification

    let signature = chunk.read_u8()?;
    if signature != 0x2f {
        return Err(ImageError::CorruptImage(
            format!("Invalid signature: 0x{:x}", signature).into(),
        ));
    }

    let mut bits = [0u8; 4];
    chunk.read_exact(&mut bits)?;
    let width = u16::from(bits[1] & 0b0011_1111) << 8 | u16::from(bits[0]);
    let height = u16::from(bits[3] & 0b0000_1111) << 10
        | u16::from(bits[2]) << 2
        | u16::from(bits[1] & 0b1100_0000) >> 6;

    Ok(Dimensions {
        width: u32::from(width) + 1,
        height: u32::from(height) + 1,
    })
}

fn read_vp8x_chunk(chunk: &mut Chunk) -> ImageResult<Dimensions> {
    // See https://developers.google.com/speed/webp/docs/webp_lossless_bitstream_specification

    let _bits = chunk.read_u32::<LittleEndian>()?;

    let mut bits = [0u8; 6];
    chunk.read_exact(&mut bits)?;
    let width = u32::from(bits[2]) << 16 | u32::from(bits[1]) << 8 | u32::from(bits[0]);
    let height = u32::from(bits[5]) << 16 | u32::from(bits[4]) << 8 | u32::from(bits[3]);

    Ok(Dimensions {
        width: width + 1,
        height: height + 1,
    })
}

fn extract_dimension(bits: [u8; 2]) -> (u16, u8) {
    let size = u16::from(bits[1] & 0b0011_1111) << 8 | u16::from(bits[0]);
    let scale = (bits[1] & 0b1100_0000) >> 6;
    (size, scale)
}
