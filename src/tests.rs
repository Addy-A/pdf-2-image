use std::path::Path;

use crate::args::{OutputFormat, RenderConfig};
use crate::matrix::Matrix;
use crate::rect::Rect;

// ---------- OutputFormat --------------------------------------------------

#[test]
fn extension_jpg() {
    assert_eq!(OutputFormat::Jpg.extension(), "jpg");
}

#[test]
fn extension_png() {
    assert_eq!(OutputFormat::Png.extension(), "png");
}

#[test]
fn extension_webp() {
    assert_eq!(OutputFormat::WebP.extension(), "webp");
}

// ---------- RenderConfig / DPI math ---------------------------------------

#[test]
fn dpi_pixel_math_150() {
    // US Letter: 612 × 792 pt at 150 DPI → 1275 × 1650 px
    let dpi: f32 = 150.0;
    let width_pts: f32 = 612.0;
    let height_pts: f32 = 792.0;

    let px_w = (width_pts / 72.0 * dpi) as i32;
    let px_h = (height_pts / 72.0 * dpi) as i32;

    assert_eq!(px_w, 1275);
    assert_eq!(px_h, 1650);
}

#[test]
fn dpi_pixel_math_300() {
    // US Letter: 612 × 792 pt at 300 DPI → 2550 × 3300 px
    let dpi: f32 = 300.0;
    let width_pts: f32 = 612.0;
    let height_pts: f32 = 792.0;

    let px_w = (width_pts / 72.0 * dpi) as i32;
    let px_h = (height_pts / 72.0 * dpi) as i32;

    assert_eq!(px_w, 2550);
    assert_eq!(px_h, 3300);
}

#[test]
fn dpi_pixel_math_72() {
    // At 72 DPI, pixel dimensions equal point dimensions
    let dpi: f32 = 72.0;
    let width_pts: f32 = 612.0;
    let height_pts: f32 = 792.0;

    let px_w = (width_pts / 72.0 * dpi) as i32;
    let px_h = (height_pts / 72.0 * dpi) as i32;

    assert_eq!(px_w, 612);
    assert_eq!(px_h, 792);
}

// ---------- Rect ----------------------------------------------------------

#[test]
fn rect_from_corners_normalizes() {
    // Reversed corners should produce same result
    let r = Rect::from_corners(642.0, 822.0, 30.0, 30.0);
    assert!((r.x - 30.0).abs() < 1e-10);
    assert!((r.y - 30.0).abs() < 1e-10);
    assert!((r.width - 612.0).abs() < 1e-10);
    assert!((r.height - 792.0).abs() < 1e-10);
}

#[test]
fn rect_right_and_top() {
    let r = Rect::new(30.0, 30.0, 612.0, 792.0);
    assert!((r.right() - 642.0).abs() < 1e-10);
    assert!((r.top() - 822.0).abs() < 1e-10);
}

#[test]
fn rect_is_outside_all_directions() {
    let trim = Rect::from_corners(30.0, 30.0, 642.0, 822.0);

    assert!(Rect::new(650.0, 100.0, 10.0, 10.0).is_outside(&trim)); // right
    assert!(Rect::new(0.0, 100.0, 10.0, 10.0).is_outside(&trim)); // left
    assert!(Rect::new(100.0, 830.0, 10.0, 10.0).is_outside(&trim)); // above
    assert!(Rect::new(100.0, 0.0, 10.0, 10.0).is_outside(&trim)); // below
}

#[test]
fn rect_straddling_is_not_outside() {
    let trim = Rect::from_corners(30.0, 30.0, 642.0, 822.0);
    assert!(!Rect::new(635.0, 100.0, 20.0, 10.0).is_outside(&trim));
}

#[test]
fn rect_fully_inside_is_not_outside() {
    let trim = Rect::from_corners(30.0, 30.0, 642.0, 822.0);
    assert!(!Rect::new(100.0, 100.0, 50.0, 50.0).is_outside(&trim));
}

// ---------- Matrix --------------------------------------------------------

#[test]
fn matrix_identity_leaves_point_unchanged() {
    let m = Matrix::identity();
    let (x, y) = m.transform_point(100.0, 200.0);
    assert!((x - 100.0).abs() < 1e-10);
    assert!((y - 200.0).abs() < 1e-10);
}

#[test]
fn matrix_translation() {
    let m = Matrix::from_values(1.0, 0.0, 0.0, 1.0, 50.0, 75.0);
    let (x, y) = m.transform_point(10.0, 20.0);
    assert!((x - 60.0).abs() < 1e-10);
    assert!((y - 95.0).abs() < 1e-10);
}

#[test]
fn matrix_concat_translations_add() {
    let t1 = Matrix::from_values(1.0, 0.0, 0.0, 1.0, 10.0, 20.0);
    let t2 = Matrix::from_values(1.0, 0.0, 0.0, 1.0, 5.0, 3.0);
    let combined = t1.concat(&t2);
    let (x, y) = combined.transform_point(0.0, 0.0);
    assert!((x - 15.0).abs() < 1e-10);
    assert!((y - 23.0).abs() < 1e-10);
}

#[test]
fn matrix_transform_rect_identity() {
    let m = Matrix::identity();
    let r = Rect::new(10.0, 20.0, 100.0, 50.0);
    let result = m.transform_rect(&r);
    assert!((result.x - 10.0).abs() < 1e-10);
    assert!((result.y - 20.0).abs() < 1e-10);
    assert!((result.width - 100.0).abs() < 1e-10);
    assert!((result.height - 50.0).abs() < 1e-10);
}

// ---------- encode (write to disk) ----------------------------------------

#[test]
fn encode_save_png_writes_file() {
    let img = image::DynamicImage::new_rgb8(10, 10);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("encode_test.png");
    crate::encode::save(&img, &path, &OutputFormat::Png).unwrap();
    assert!(path.exists());
    assert!(std::fs::metadata(&path).unwrap().len() > 0);
    std::fs::remove_file(&path).ok();
}

#[test]
fn encode_save_jpg_writes_file() {
    let img = image::DynamicImage::new_rgb8(10, 10);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("encode_test.jpg");
    crate::encode::save(&img, &path, &OutputFormat::Jpg).unwrap();
    assert!(path.exists());
    assert!(std::fs::metadata(&path).unwrap().len() > 0);
    std::fs::remove_file(&path).ok();
}

#[test]
fn encode_save_webp_writes_file() {
    let img = image::DynamicImage::new_rgb8(10, 10);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("encode_test.webp");
    crate::encode::save(&img, &path, &OutputFormat::WebP).unwrap();
    assert!(path.exists());
    assert!(std::fs::metadata(&path).unwrap().len() > 0);
    std::fs::remove_file(&path).ok();
}

// ---------- process_pdf (integration, requires PDFium) --------------------

#[test]
#[ignore] // requires PDFium native library on system
fn process_pdf_renders_pages() {
    let input = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/pdf_test_data_print_v2.pdf");
    let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result");
    std::fs::create_dir_all(&out_dir).unwrap();

    let config = RenderConfig { dpi: 150 };
    let format = OutputFormat::Jpg;

    crate::process::process_pdf(&input, &out_dir, &config, &format).unwrap();

    let output_file = out_dir.join("pdf_test_data_print_v2-p001.jpg");
    assert!(output_file.exists(), "expected rendered page file to exist");
    assert!(std::fs::metadata(&output_file).unwrap().len() > 0);
    std::fs::remove_file(&output_file).ok();
}

#[test]
#[ignore]
fn bench_process_pdf_150dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_ -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-001.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "150"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf 150dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}

#[test]
#[ignore]
fn bench_process_pdf_300dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_ -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-001.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "300"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf 300dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}

#[test]
#[ignore]
fn bench_process_pdf_600dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_ -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-001.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "600"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf 600dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}

#[test]
#[ignore]
fn bench_process_pdf_book_150dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_process_pdf_book -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-002-book.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "150"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf_book 150dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}

#[test]
#[ignore]
fn bench_process_pdf_book_300dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_process_pdf_book -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-002-book.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "300"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf_book 300dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}

#[test]
#[ignore]
fn bench_process_pdf_book_600dpi() {
    // run with: DYLD_LIBRARY_PATH=$HOME/.cargo/bin cargo test bench_process_pdf_book -- --nocapture --ignored --test-threads=1
    let input = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test/test_assets/secret_fixtures/fixture-002-book.pdf");
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("test/test_result/secret_results");
    std::fs::create_dir_all(&output).unwrap();

    let start = std::time::Instant::now();
    let status = std::process::Command::new("p2i")
        .args(["-dpi", "600"])
        .arg(&input)
        .arg(&output)
        .status()
        .expect("p2i not found - run `cargo install -- path .` first");
    let elapsed = start.elapsed();

    assert!(status.success(), "p2i exited with failure");
    println!("process_pdf_book 600dpi: {elapsed:?}");
    assert!(elapsed.as_secs() < 60, "render took too long: {elapsed:?}")
}
