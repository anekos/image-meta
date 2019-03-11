



pub struct ImageMeta {
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
