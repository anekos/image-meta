
# image-meta

[![Build status](https://travis-ci.org/anekos/image-meta.svg?branch=master)](https://travis-ci.org/anekos/image-meta)
[![crates.io](https://img.shields.io/crates/v/image-meta.svg)](https://crates.io/crates/image-meta)
[![Documentation](https://docs.rs/image-meta/badge.svg)](https://docs.rs/image-meta)

Image meta data inspector for rust


# Supported formats

- [APNG](https://en.wikipedia.org/wiki/APNG)
- [BMP](https://en.wikipedia.org/wiki/BMP_file_format)
- [GIF](https://en.wikipedia.org/wiki/GIF)
- [JPEG](https://en.wikipedia.org/wiki/JPEG)
- [PNG](https://en.wikipedia.org/wiki/Portable_Network_Graphics)
- [WebP](https://en.wikipedia.org/wiki/WebP)
- [HDR](https://en.wikipedia.org/wiki/RGBE_image_format)
- [QOI](https://en.wikipedia.org/wiki/QOI_\(image_format\))


# Usage

```
[dependencies]
image-meta = "*"
```

```rust,ignore
use image_meta;

fn main() {
  let meta = image_meta::load_from_file("test-files/paw.png").unwrap();
  println!("dims: {}x{}", meta.dimensions.width, meta.dimensions.height);
  println!("animation: {:?}", meta.is_animation());
  println!("format: {:?}", meta.format);
}
```
