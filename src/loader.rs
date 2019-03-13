
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
    use crate::types::ImageMeta;


    fn test_load_x<F>(extension: &str, animation: bool, load: F)
    where F: Fn(&mut BufReader<File>) -> ImageResult<ImageMeta> {
        let suffix = if animation { "-animation" } else { "" };
        let file = File::open(format!("test-files/paw{}.{}", suffix, extension)).unwrap();
        let mut file = BufReader::new(file);
        let meta = load(&mut file).unwrap();
        assert_eq!(meta.dimensions.width, 507);
        assert_eq!(meta.dimensions.height, 370);
        if animation {
            assert_eq!(meta.animation_frames, Some(4));
        } else {
            assert_eq!(meta.animation_frames, None);
        }
    }

    #[test]
    fn test_load() {
        test_load_x("gif", false, loader::gif::load);
        test_load_x("jpg", false, loader::jpeg::load);
        test_load_x("png", false, loader::png::load);

        test_load_x("gif", true, loader::gif::load);
        test_load_x("png", true, loader::png::load);

        test_load_x("gif", false, loader::load);
        test_load_x("jpg", false, loader::load);
        test_load_x("png", false, loader::load);

        test_load_x("gif", true, loader::load);
        test_load_x("png", true, loader::load);
    }

    #[test]#[should_panic(expected="Unsupported")]
    fn test_load_bad() {
        loader::load_from_file("test-files/bad.dat").unwrap();
    }
}
