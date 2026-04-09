use std::path::PathBuf;

pub enum OutputFormat {
    Jpg,
    Png,
    WebP,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Jpg => "jpg",
            OutputFormat::Png => "png",
            OutputFormat::WebP => "webp",
        }
    }
}

pub struct RenderConfig {
    pub dpi: u32,
}

pub enum InputMode {
    Single(PathBuf),
    Batch(Vec<PathBuf>),
}