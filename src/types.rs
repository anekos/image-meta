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

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display)]
pub enum Format {
    Bmp,
    Gif,
    Jpeg,
    Png,
    Webp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, strum::Display)]
pub enum ColorMode {
    Grayscale,
    Indexed,
    Rgb,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    pub alpha_channel: bool,
    pub mode: ColorMode,
    pub resolution: u8,
}

impl ImageMeta {
    pub fn is_animation(&self) -> bool {
        self.animation_frames.is_some()
    }
}
