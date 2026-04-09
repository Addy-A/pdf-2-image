pub mod args;
mod encode;
pub mod matrix;
pub mod process;
pub mod rect;
mod render;

pub use args::{InputMode, OutputFormat, RenderConfig};
pub use process::process_pdf;

#[cfg(test)]
mod tests;
