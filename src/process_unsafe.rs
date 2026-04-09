use std::path::Path;

use pdfium_render::prelude::*;
use rayon::prelude::*;

use crate::args::{OutputFormat, RenderConfig};
use crate::encode;
use crate::render::render_page;

/// Processes a PDF file by rendering each page as an image and saving it to the specified output directory.
///
/// This function loads a PDF document, renders each page according to the provided configuration,
/// and saves the rendered pages as individual images in the requested format.
///
/// # Arguments
///
/// * `input_path` - Path to the input PDF file to be processed
/// * `output_dir` - Directory where the rendered page images will be saved
/// * `config` - Rendering configuration specifying quality, scale, and other rendering parameters
/// * `format` - Output format specification determining the image format and encoding settings
///
/// # Returns
///
/// * `Ok(())` if processing completes successfully
/// * `Err(PdfiumError)` if any error occurs during PDF loading, rendering, or file operations
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use pdf_to_image::{process_pdf, RenderConfig, OutputFormat};
///
/// let input = Path::new("document.pdf");
/// let output = Path::new("./output/");
/// let config = RenderConfig { dpi: 150 };
/// let format = OutputFormat::Png;
///
/// process_pdf(input, output, &config, &format).unwrap();
/// ```
///
/// # File Naming
///
/// Rendered pages are saved with filenames following the pattern: `{original_filename}-p{page_number}.{extension}`
/// where page numbers are zero-padded to 3 digits (e.g., "document-p001.png").
///
/// # Thread Safety
///
/// This function uses parallel processing to render multiple pages concurrently for improved performance.
pub fn process_pdf(
    input_path: &Path,
    output_dir: &Path,
    config: &RenderConfig,
    format: &OutputFormat,
) -> Result<(), PdfiumError> {
    let dylib_name = if cfg!(target_os = "windows") {
        "pdfium.dll"
    } else if cfg!(target_os = "macos") {
        "libpdfium.dylib"
    } else {
        "libpdfium.so"
    };

    let pdfium = Pdfium::new(
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join(dylib_name)))
            .and_then(|lib| Pdfium::bind_to_library(lib).ok())
            .map_or_else(|| Pdfium::bind_to_system_library(), Ok)?,
    );
    let document = pdfium.load_pdf_from_file(input_path, None)?;
    let stem = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let page_count = document.pages().len();
    let indices: Vec<i32> = (0..page_count).collect();

    const PARALLEL_THRESHOLD: usize = 4;

    if page_count <= PARALLEL_THRESHOLD as i32 {
        for &index in &indices {
            let page = document.pages().get(index).unwrap();
            let image = render_page(&page, config).unwrap();
            let name = format!("{}-p{:03}.{}", stem, index + 1, format.extension());
            let path = output_dir.join(name);
            encode::save(&image, &path, format).unwrap();
        }
    } else {
        let input_path_owned = input_path.to_path_buf();
        let output_dir_owned = output_dir.to_path_buf();
        let pdfium = std::sync::Arc::new(&pdfium);
        drop(document);
        indices.par_iter().for_each(|&index| {
            let document = pdfium.load_pdf_from_file(&input_path_owned, None).unwrap();
            let page = document.pages().get(index).unwrap();
            let image = render_page(&page, config).unwrap();
            let name = format!("{}-p{:03}.{}", stem, index + 1, format.extension());
            let path = output_dir_owned.join(name);
            encode::save(&image, &path, format).unwrap();
        });
    }

    Ok(())
}
