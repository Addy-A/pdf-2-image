use image::DynamicImage;
use pdfium_render::prelude::*;

use crate::args::RenderConfig;

/// Rasterize a single PDF page to an in-memory image.
///
/// Pixel dimensions are computed from the page's point dimensions:
///   pixel_width  = page_width_pts  / 72.0 * dpi
///   pixel_height = page_height_pts / 72.0 * dpi
///
/// Annotations and form data are rendered (matching what a PDF viewer shows).
pub fn render_page(page: &PdfPage, config: &RenderConfig) -> Result<DynamicImage, PdfiumError> {
    let dpi = config.dpi as f32;

    let pixel_width = (page.width().value / 72.0 * dpi) as i32;
    let pixel_height = (page.height().value / 72.0 * dpi) as i32;

    let render_cfg = PdfRenderConfig::new()
        .set_target_width(pixel_width)
        .set_maximum_height(pixel_height)
        .render_annotations(true)
        .render_form_data(true);

    page.render_with_config(&render_cfg)
        .and_then(|bitmap| bitmap.as_image())
}