



#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageMeta {
    pub animation_frames: Option<usize>,
    pub dimensions: Dimensions,
    pub format: Format,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Format {
    Gif,
    Jpeg,
    Png,
}
