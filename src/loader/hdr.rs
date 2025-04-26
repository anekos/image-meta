use std::io::{BufRead, Seek};

use crate::errors::{ImageError, ImageResult};
use crate::types::{Color, Dimensions, Format, ImageMeta};

const SIGNATURE: [u8; 11] = [
    0x23, 0x3f, 0x52, 0x41, 0x44, 0x49, 0x41, 0x4e, 0x43, 0x45, 0x0a,
];

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;

    let (dimensions, color) = read_header(image)?;

    Ok(ImageMeta {
        animation_frames: None,
        color,
        dimensions,
        format: Format::Hdr,
    })
}

fn read_signature<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult {
    let mut signature = [0u8; 11];
    image.read_exact(&mut signature)?;
    if SIGNATURE != signature {
        return Err(ImageError::InvalidSignature);
    }
    Ok(())
}

fn read_header<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<(Dimensions, Color)> {
    use crate::types::ColorMode::*;

    let color = Color {
        mode: Rgb,
        alpha_channel: false,
        resolution: 32,
    };
    while let Some(Ok(line)) = image.lines().next() {
        // Skip empty lines and comments
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Try to parse the line as a key-value pair
        if let Some((key, value)) = line.split_once("=") {
            match key {
                "FORMAT" => match value {
                    "32-bit_rle_rgbe" => {}
                    "32-bit_rle_xyze" => {}
                    _ => {
                        return Err(ImageError::CorruptImage(
                            format!("Unsupported format: {}", value).into(),
                        ));
                    }
                },
                "PRIMARIES" => {}
                _ => {}
            }
        }
        // Else, we have reached the resolution line
        else {
            let mut iter = line.split_whitespace();
            let c1_tag = iter
                .next()
                .ok_or(ImageError::CorruptImage("Error parsing dimension".into()))?;
            let c1_str = iter
                .next()
                .ok_or(ImageError::CorruptImage("Error parsing dimension".into()))?;
            let c2_tag = iter
                .next()
                .ok_or(ImageError::CorruptImage("Error parsing dimension".into()))?;
            let c2_str = iter
                .next()
                .ok_or(ImageError::CorruptImage("Error parsing dimension".into()))?;
            match (c1_tag, c2_tag) {
                ("-Y", "+X") => {
                    // Common orientation (left-right, top-down)
                    // c1_str is height, c2_str is width
                    let height = c1_str.parse::<u32>().map_err(|err| {
                        ImageError::CorruptImage(format!("Error parsing height: {err}").into())
                    })?;
                    let width = c2_str.parse::<u32>().map_err(|err| {
                        ImageError::CorruptImage(format!("Error parsing width: {err}").into())
                    })?;
                    return Ok((Dimensions { width, height }, color));
                }
                _ => return Err(ImageError::Unsupported),
            }
        }
    }
    Err(ImageError::Unsupported)
}
