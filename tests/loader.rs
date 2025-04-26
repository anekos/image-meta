use std::fs::File;
use std::io::{BufReader, Cursor};

use image_meta::ColorMode::*;
use image_meta::*;

const DIMS: Dimensions = Dimensions {
    width: 507,
    height: 370,
};

fn load_file<F>(suffix: &str, loader: F) -> ImageMeta
where
    F: Fn(&mut BufReader<File>) -> ImageResult<ImageMeta>,
{
    let file = File::open(format!("test-files/paw{}", suffix)).unwrap();
    let mut file = BufReader::new(file);
    loader(&mut file).unwrap()
}

#[test]
fn test_each_loader() {
    assert_eq!(
        load_file(".bmp", bmp::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Bmp,
        }
    );
    assert_eq!(
        load_file(".gif", gif::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Indexed,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Gif,
        }
    );
    assert_eq!(
        load_file(".jpg", jpeg::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Jpeg,
        }
    );
    assert_eq!(
        load_file(".png", png::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Png,
        }
    );
    assert_eq!(
        load_file(".webp", webp::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Webp,
        }
    );
    assert_eq!(
        load_file(".hdr", hdr::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 32
            },
            dimensions: DIMS,
            format: Format::Hdr,
        }
    );
    assert_eq!(
        load_file(".lossless.webp", webp::load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Webp,
        }
    );
}

#[test]
fn test_each_loader_for_animation() {
    assert_eq!(
        load_file("-animation.gif", gif::load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Indexed,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Gif,
        }
    );
    assert_eq!(
        load_file("-animation.png", png::load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Png,
        }
    );
    assert_eq!(
        load_file("-animation.webp", webp::load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Webp,
        }
    );
}

#[test]
fn test_guess_loader() {
    assert_eq!(
        load_file(".bmp", load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Bmp,
        }
    );
    assert_eq!(
        load_file(".gif", load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Indexed,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Gif,
        }
    );
    assert_eq!(
        load_file(".jpg", load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Jpeg,
        }
    );
    assert_eq!(
        load_file(".png", load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Png,
        }
    );
    assert_eq!(
        load_file(".webp", load),
        ImageMeta {
            animation_frames: None,
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Webp,
        }
    );
}

#[test]
fn test_guess_loader_for_animation() {
    assert_eq!(
        load_file("-animation.gif", load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Indexed,
                alpha_channel: false,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Gif,
        }
    );
    assert_eq!(
        load_file("-animation.png", load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Png,
        }
    );
    assert_eq!(
        load_file("-animation.webp", load),
        ImageMeta {
            animation_frames: Some(4),
            color: Color {
                mode: Rgb,
                alpha_channel: true,
                resolution: 8
            },
            dimensions: DIMS,
            format: Format::Webp,
        }
    );
}

#[test]
#[should_panic(expected = "Unsupported")]
fn test_load_bad() {
    load_from_file("test-files/bad.dat").unwrap();
}

#[test]
fn test_load_webp_corrupt_filesize() {
    // Empty WEBP
    let mut file = Cursor::new(b"RIFF\x00\x00\x00\x00WEBP");
    assert!(webp::load(&mut file).is_err());

    // Empty and truncated
    let mut file = Cursor::new(b"RIFF\x00\x00\x00\x00");
    assert!(webp::load(&mut file).is_err());
}

#[test]
fn test_load_webp_corrupt_riff_chunk() {
    // This caused subtraction overflow while reading a chunk
    let mut file = Cursor::new(b"RIFF\x08\x00\x00\x00WEBPVP8 \x00\x00\x00\x00");
    assert!(webp::load(&mut file).is_err());
}
