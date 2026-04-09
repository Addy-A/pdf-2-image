use crate::args::OutputFormat;
use image::DynamicImage;
use std::path::Path;

/// Saves a dynamic image to the specified path in the given format.
///
/// This function takes a `DynamicImage` and saves it to disk using the specified
/// output format. It supports PNG, JPEG, and WebP formats.
///
/// # Arguments
///
/// * `image` - A reference to the `DynamicImage` to be saved
/// * `path` - A reference to the `Path` where the image should be saved
/// * `format` - A reference to the `OutputFormat` specifying the desired save format
///
/// # Returns
///
/// * `Ok(())` if the image was successfully saved
/// * `Err(image::ImageError)` if there was an error during the save operation
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use image::{DynamicImage, ImageFormat};
///
/// // Assuming you have a dynamic image and want to save it as PNG
/// // save(&image, Path::new("output.png"), &OutputFormat::Png);
/// ```
pub fn save(
    image: &DynamicImage,
    path: &Path,
    format: &OutputFormat,
) -> Result<(), image::ImageError> {
    let image_format = match format {
        OutputFormat::Png => image::ImageFormat::Png,
        OutputFormat::Jpg => image::ImageFormat::Jpeg,
        OutputFormat::Tiff => image::ImageFormat::Tiff,
        OutputFormat::WebP => {
            let encoder = webp::Encoder::from_image(image).unwrap();
            let encoded = encoder.encode(80.0);
            std::fs::write(path, &*encoded)?;
            return Ok(());
        }
    };
    image.save_with_format(path, image_format)
}
