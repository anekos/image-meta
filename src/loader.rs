
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;

pub mod gif;
pub mod jpeg;
pub mod png;

use crate::errors::ImageError::InvalidSignature;
use crate::errors::ImageResult;
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

pub fn load<R: BufRead + Seek>(image: &mut R) -> ImageResult<ImageMeta> {
    try_to_load!(gif, image);
    try_to_load!(jpeg, image);
    png::load(image)
}

pub fn load_from_file<T: AsRef<Path>>(file: &T) -> ImageResult<ImageMeta> {
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


    fn test_load_x<F>(extension: &str, load: F)
    where F: Fn(&mut BufReader<File>) -> ImageResult<ImageMeta> {
        let file = File::open(format!("test-files/paw.{}", extension)).unwrap();
        let mut file = BufReader::new(file);
        let meta = load(&mut file).unwrap();
        assert_eq!(meta.dimensions.width, 507);
        assert_eq!(meta.dimensions.height, 370);
    }

    #[test]
    fn test_load() {
        test_load_x("gif", loader::gif::load);
        test_load_x("jpg", loader::jpeg::load);
        test_load_x("png", loader::png::load);
        test_load_x("gif", loader::load);
        test_load_x("jpg", loader::load);
        test_load_x("png", loader::load);
    }
}
