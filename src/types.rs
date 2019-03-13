



pub struct ImageMeta {
    pub animation_frames: Option<usize>,
    pub dimensions: Dimensions,
    pub format: Format,
}

pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

pub enum Format {
    Gif,
    Jpeg,
    Png,
}
