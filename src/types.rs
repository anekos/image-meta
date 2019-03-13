



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageMeta {
    pub animation_frames: Option<usize>,
    pub color: Color,
    pub dimensions: Dimensions,
    pub format: Format,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    Gif,
    Jpeg,
    Png,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Grayscale(u8),
    GrayscaleA(u8),
    Palette(u8),
    Rgb(u8),
    RgbA(u8),
}

impl ImageMeta {
    pub fn is_animation(&self) -> bool {
        self.animation_frames.is_some()
    }
}
