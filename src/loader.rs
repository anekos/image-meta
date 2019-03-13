
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;

pub mod gif;
pub mod jpeg;
pub mod png;

use crate::errors::ImageError::InvalidSignature;
use crate::errors::{ImageError, ImageResult};
use crate::types::ImageMeta;



macro_rules! try_to_load {
    ($image_type:ident, $image:ident) => {
        match $image_type::load($image) {
            Ok(meta) => return Ok(meta),
            Err(InvalidSignature) => {
                $image.seek(SeekFrom::Start(0))?;
            },
            otherwise => return otherwise,
        }
    }
}

pub fn load<R: ?Sized + BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    try_to_load!(gif, image);
    try_to_load!(jpeg, image);
    try_to_load!(png, image);
    Err(ImageError::Unsupported)
}

pub fn load_from_file<T: ?Sized + AsRef<Path>>(file: &T) -> ImageResult<ImageMeta> {
    let file = File::open(file.as_ref())?;
    let mut file = BufReader::new(file);
    load(&mut file)
}



#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use crate::errors::ImageResult;
    use crate::loader;
    use crate::types::{Color, Dimensions, Format, ImageMeta};

    const DIMS: Dimensions = Dimensions { width: 507, height: 370 };


    fn load<F>(extension: &str, animation: bool, loader: F) -> ImageMeta
    where F: Fn(&mut BufReader<File>) -> ImageResult<ImageMeta> {
        let suffix = if animation { "-animation" } else { "" };
        let file = File::open(format!("test-files/paw{}.{}", suffix, extension)).unwrap();
        let mut file = BufReader::new(file);
        loader(&mut file).unwrap()
    }

    #[test]
    fn test_each_loader() {
        assert_eq!(
            load("gif", false, loader::gif::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Palette(8),
                dimensions: DIMS,
                format: Format::Gif,
            });
        assert_eq!(
            load("jpg", false, loader::jpeg::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Rgb(8),
                dimensions: DIMS,
                format: Format::Jpeg,
            });
        assert_eq!(
            load("png", false, loader::png::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Rgb(8),
                dimensions: DIMS,
                format: Format::Png,
            });
    }

    #[test]
    fn test_each_loader_for_animation() {
        assert_eq!(
            load("gif", true, loader::gif::load),
            ImageMeta {
                animation_frames: Some(4),
                color: Color::Palette(8),
                dimensions: DIMS,
                format: Format::Gif,
            });
        assert_eq!(
            load("png", true, loader::png::load),
            ImageMeta {
                animation_frames: Some(4),
                color: Color::RgbA(8),
                dimensions: DIMS,
                format: Format::Png,
            });
    }

    #[test]
    fn test_guess_loader() {
        assert_eq!(
            load("gif", false, loader::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Palette(8),
                dimensions: DIMS,
                format: Format::Gif,
            });
        assert_eq!(
            load("jpg", false, loader::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Rgb(8),
                dimensions: DIMS,
                format: Format::Jpeg,
            });
        assert_eq!(
            load("png", false, loader::load),
            ImageMeta {
                animation_frames: None,
                color: Color::Rgb(8),
                dimensions: DIMS,
                format: Format::Png,
            });
    }

    #[test]
    fn test_guess_loader_for_animation() {
        assert_eq!(
            load("gif", true, loader::load),
            ImageMeta {
                animation_frames: Some(4),
                color: Color::Palette(8),
                dimensions: DIMS,
                format: Format::Gif,
            });
        assert_eq!(
            load("png", true, loader::load),
            ImageMeta {
                animation_frames: Some(4),
                color: Color::RgbA(8),
                dimensions: DIMS,
                format: Format::Png,
            });
    }

    #[test]#[should_panic(expected="Unsupported")]
    fn test_load_bad() {
        loader::load_from_file("test-files/bad.dat").unwrap();
    }
}
