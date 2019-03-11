
use std::io::{BufRead, Cursor, Seek, SeekFrom};

use byteorder::{ReadBytesExt, BigEndian};

use crate::errors::{ImageError, ImageResult, ImageResultU};
use crate::types::{Dimensions, Format, ImageMeta};



const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];


pub fn load<R: BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    read_signature(image)?;

    let dimensions = read_header(image)?;

    Ok(ImageMeta {
        dimensions,
        format: Format::Png,
    })
}

fn read_signature<R: BufRead + Seek>(image: &mut R) -> ImageResultU {
    let mut signature = [0u8;8];
    image.read_exact(&mut signature)?;
    if SIGNATURE != signature {
        return Err(ImageError::InvalidSignature);
    }
    Ok(())
}

fn read_header<R: BufRead + Seek>(image: &mut R) -> ImageResult<Dimensions> {
    let (chunk_name, chunk_data) = read_chunk(image)?;
    if chunk_name != *b"IHDR" {
        return Err(ImageError::CorruptImage("Not IHDR"));
    }
    let mut chunk_data = Cursor::new(chunk_data);

    let width = chunk_data.read_u32::<BigEndian>()?;
    let height = chunk_data.read_u32::<BigEndian>()?;

    // 0: bit_depth, 1: color_type, 2: compression_method, 3: filter_method, 4: interlace_method
    // image.seek(SeekFrom::Start(5))?;
    // Skip Chunk

    Ok(Dimensions { height, width })
}

fn read_chunk<R: BufRead + Seek>(image: &mut R) -> ImageResult<([u8;4], Vec<u8>)> {
    let length = image.read_u32::<BigEndian>()?;
    let mut chunk_name = [0u8;4];
    image.read_exact(&mut chunk_name)?;
    let mut result = vec![0u8; length as usize];
    image.read_exact(&mut result)?;
    // Skip CRC
    image.seek(SeekFrom::Current(4))?;
    Ok((chunk_name, result))
}



#[cfg(test)]
mod tests {
    use crate::loader::png::load;

    #[test]
    fn test_load() {
        let file = std::fs::File::open("test-files/paw.png").unwrap();
        let mut file = std::io::BufReader::new(file);
        let meta = load(&mut file).unwrap();
        assert_eq!(meta.dimensions.width, 507);
        assert_eq!(meta.dimensions.height, 370);
    }
}
