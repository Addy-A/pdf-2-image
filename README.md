# PDF To Image

Rasterize PDF pages to JPG, PNG, WebP, or TIFF at a caller-specified DPI.

## Constraints

| Rule                     | Detail                                                                                           |
|--------------------------|--------------------------------------------------------------------------------------------------|
| **No content modification** | PDFs are opened read-only. No content streams are decoded, filtered, or re-encoded.          |
| **Prepress-aware DPI**   | Default 150 DPI for proofing. `-prepress` flag forces 300 DPI + JPG.                             |
| **No paid services**     | No cloud APIs, no external runtime services. PDFium is a local native library.                   |
| **Standalone**           | No dependency on the other tools in the series at runtime.                                       |
| **Single-threaded rendering** | Pages are rendered sequentially. PDFium's C library uses process-level global state that is not safe for concurrent document access. See [Known Limitations](#known-limitations). |

## Status

**Beta.** Pipeline validated against production-level prepress PDFs across
multiple document types, page counts, and DPI settings. Rasterization, encoding,
and CLI argument parsing are functional. See [Known Limitations](#known-limitations)
for current tradeoffs.

---

## Dependencies

- **Language:** Rust (2024 edition)
- **Crates:**
    - [`pdfium-render 0.9.0`](https://crates.io/crates/pdfium-render)
      ([source](https://github.com/ajrcarey/pdfium-render)) -- PDF rasterization
      via Google's PDFium engine
    - [`image 0.25.10`](https://crates.io/crates/image)
      ([source](https://github.com/image-rs/image)) -- bitmap encoding to
      JPG, PNG, TIFF
    - [`webp 0.3.1`](https://crates.io/crates/webp)
      ([source](https://github.com/jaredforth/webp)) -- lossy WebP encoding
      via libwebp bindings (replaces `image` crate's lossless-only WebP encoder)
    - [`rayon 1.11.0`](https://crates.io/crates/rayon)
      ([source](https://github.com/rayon-rs/rayon)) -- retained as a dependency;
      parallel rendering is not currently active (see [Known Limitations](#known-limitations))
- No paid services or external tooling required at runtime.

**Runtime native dependency:** PDFium shared library (`libpdfium.so` /
`pdfium.dll` / `libpdfium.dylib`). Must be present on the system or placed
beside the binary. Prebuilt binaries are available from the
[pdfium-binaries](https://github.com/bblanchon/pdfium-binaries) project.

No `clap`. Argument parsing is manual, consistent with `ptrim` and `prsz`.

---

## Usage

### Single-file mode

```bash
p2i [-dpi <n>] [-format <jpg|jpeg|png|webp|tiff>] [-prepress] <input.pdf> [output/dir]
```

| Flag / Argument     | Description                                                                               |
|----------------------|------------------------------------------------------------------------------------------|
| `<input.pdf>`        | Path to the source PDF.                                                                  |
| `[output/dir]`       | Output directory (default: same directory as input).                                     |
| `-dpi <n>`           | Render DPI (default: 150).                                                               |
| `-format <fmt>`      | Output format: `jpg`, `jpeg`, `png`, `webp`, `tiff` (default: `jpg`).                   |
| `-prepress`          | Shorthand for `-dpi 300 -format jpg`. Intended for print-ready output.                   |

```bash
# Single file → JPG at 150 DPI (default)
p2i artwork.pdf

# Single file → PNG at 300 DPI
p2i -dpi 300 -format png artwork.pdf output/

# Single file → TIFF at 300 DPI (RIP / Photoshop hand-off)
p2i -dpi 300 -format tiff artwork.pdf output/

# Prepress shorthand (300 DPI, JPG)
p2i -prepress artwork.pdf output/
```

### Batch mode

```bash
p2i [-dpi <n>] [-format <fmt>] [input-1.pdf, input-2.pdf, ...] [output/dir]
```

Pass a comma-separated list of input paths enclosed in square brackets, with
an optional output directory as the final argument.

| Argument             | Description                                                                                                    |
|----------------------|----------------------------------------------------------------------------------------------------------------|
| `[input-1.pdf, …]`  | One or more PDF paths separated by commas, wrapped in `[` and `]`. Spaces around commas and paths are ignored. |
| `[output/dir]`       | Directory where every rendered file is written (optional). Defaults to the same directory as each input file.  |

Each page is written as `<stem>-p<NNN>.<ext>` inside the output directory.

```bash
# Batch → WebP at 150 DPI (lossy, for web proof review)
p2i -format webp '[art-1.pdf, art-2.pdf, art-3.pdf]' output/previews/

# Batch — each rendered file beside its input
p2i -prepress '[art-1.pdf, art-2.pdf, art-3.pdf]'
```

> **zsh / bash note:** Square brackets are reserved glob syntax in most shells.
> Escape them with backslashes on the command line:
>
> ```bash
> p2i -prepress \[art-1.pdf, art-2.pdf, art-3.pdf\] output/
> ```
>
> Alternatively, single-quote the entire bracket block:
>
> ```bash
> p2i -prepress '[art-1.pdf, art-2.pdf, art-3.pdf]' output/
> ```

### Install

#### 1. Install the PDFium native library

`p2i` requires Google's PDFium shared library at runtime. Prebuilt binaries are
published by the [pdfium-binaries](https://github.com/bblanchon/pdfium-binaries)
project:

1. Go to the [pdfium-binaries releases page](https://github.com/bblanchon/pdfium-binaries/releases).
2. Download the archive for your platform:
   - **Windows x64:** `pdfium-win-x64.tgz`
   - **macOS arm64:** `pdfium-mac-arm64.tgz`
   - **macOS x64:** `pdfium-mac-x64.tgz`
   - **Linux x64:** `pdfium-linux-x64.tgz`
3. Extract the archive. The shared library is inside the `bin/` (Windows) or
   `lib/` (macOS / Linux) folder:
   - **Windows:** `pdfium.dll`
   - **macOS:** `libpdfium.dylib`
   - **Linux:** `libpdfium.so`
4. Place the library where the OS can find it:

| Platform | Option A (beside binary)                  | Option B (system-wide)           |
|----------|-------------------------------------------|----------------------------------|
| Windows  | Copy `pdfium.dll` next to `p2i.exe`       | Place in any directory on `PATH` |
| macOS    | Copy `libpdfium.dylib` next to `p2i`      | Copy to `/usr/local/lib/`        |
| Linux    | Copy `libpdfium.so` next to `p2i`         | Copy to `/usr/local/lib/` and run `sudo ldconfig` |

> **Tip (Windows):** If you installed `p2i` via `cargo install`, the binary
> lives at `%USERPROFILE%\.cargo\bin\p2i.exe`. Drop `pdfium.dll` in that same
> folder.

#### 2. Install the `p2i` binary

```bash
cargo install --path .
```

This places the binary at `~/.cargo/bin/p2i`.

#### Verify

```bash
p2i --version
p2i artwork.pdf
```

If the PDFium library is missing or not found, `p2i` will report a
`LoadLibraryError` at runtime.

### Build

```bash
cargo build --release
```

### Run (without installing)

```bash
cargo run --release -- -prepress artwork.pdf output/
```

### Test

```bash
cargo test -- --nocapture
```

---

## How It Works

### High-level pipeline

1. **Parse args** — determine input mode (single file or batch bracket syntax),
   output directory, format (`jpg` / `png` / `webp`), and DPI.
2. **For each input PDF:**
   a. Bind to the PDFium shared library via `Pdfium::bind_to_system_library()`.
   b. Load the document with `pdfium.load_pdf_from_file(path, None)`.
   c. Collect page indices: `0..doc.pages().len()`.
   d. Render all pages in parallel via `rayon::par_iter`.
3. **Per page (in parallel):**
   a. Open the page via `doc.pages().get(idx)`.
   b. Compute pixel dimensions from page size in points and target DPI:
      `px = points / 72.0 * dpi`.
   c. Render to bitmap using `PdfRenderConfig` with annotations and form
      data enabled.
   d. Convert bitmap to `DynamicImage` via `bitmap.as_image()`.
4. **Encode** the `DynamicImage` to the target format and write to disk.
5. **Output naming** follows the series convention:
   `<stem>-p<NNN>.<ext>` (e.g. `artwork-p001.jpg`).

### Why pdfium-render

`lopdf` (used in the sibling tools) manipulates the PDF object graph but does
not render. Rendering requires a full PDF engine. `pdfium-render` wraps Google's
PDFium — the engine used in Chrome — which handles transparency, blending modes,
Type 3 fonts, spot color (partial), and the class of complex InDesign-exported
files the series targets.

### DPI and the points conversion

PDF page dimensions are in **points** (1 pt = 1/72 inch). To render at a
target DPI:

```
pixel_width  = page_width_pts  / 72.0 * dpi
pixel_height = page_height_pts / 72.0 * dpi
```

A US Letter page (612 × 792 pt) at 150 DPI → 1275 × 1650 px.
A US Letter page at 300 DPI → 2550 × 3300 px.

This vocabulary (DPI, not pixels) is what prepress operators work in natively.

### Parallelism model

Rasterization is CPU-bound. The bottleneck is not disk I/O — it is rendering.
Pages are currently rendered **sequentially** within each document. PDFium's
underlying C library maintains process-level global state: concurrent calls to
`load_pdf_from_file` across threads produce `PdfiumLibraryInternalError` and
malloc corruption regardless of the `thread_safe` feature flag (which only
adds Rust's `Send + Sync` bounds, not C-library thread safety).

An experimental parallel branch is preserved in `process_unsafe.rs` for future
work. If a version of `pdfium-render` introduces genuine thread-safe C bindings,
the `par_iter` approach in that file is the intended upgrade path.

Multiple input files in batch mode are also processed sequentially — one
PDFium context, one document, one page at a time.

---

## PDF Background

PDF page dimensions are always expressed in **points** (1/72 inch), with the
coordinate origin at the bottom-left corner of the page.

The `pdfium-render` crate accepts pixel dimensions for its render target.
The conversion is:

```
pixel_dimension = point_dimension / 72.0 * dpi
```

Unlike the sibling tools, `p2i` does not manipulate any PDF object directly —
it opens documents read-only and delegates all rendering decisions to PDFium.

### Page boxes

This tool respects whatever the PDF viewer would render by default — PDFium uses
the **CropBox** as the visible rendering boundary, falling back to the MediaBox if
no CropBox is defined. The TrimBox is not consulted during rasterization (it is a
metadata box, not a rendering boundary). To produce output cropped to the
TrimBox, run `prsz -t` on the input first, then pass the result to `p2i`.

---

## Project Structure

```
Cargo.toml
README.md
src/
    lib.rs          -- crate root: module declarations and public re-exports
    main.rs         -- binary entry point, arg parsing
    args.rs         -- CLI argument types (InputMode, OutputFormat, RenderConfig)
    render.rs       -- pdfium-render page rasterization → DynamicImage
    encode.rs       -- image crate encode DynamicImage → file (JPG/PNG/WebP)
    process.rs          -- top-level pipeline (load, dispatch pages, collect output)
    process_unsafe.rs   -- experimental parallel rendering branch (non-functional; see Known Limitations)
    rect.rs         -- Rect struct (consistent with sibling repos)
    matrix.rs       -- Matrix struct (consistent with sibling repos)
    tests.rs        -- unit and integration tests (compiled only in test builds)
test/
    test_assets/    -- PDF fixtures for integration tests
    test_result/    -- output directory for test runs (gitignored)
```

### Key types and functions

| Item           | Location     | Purpose                                             |
|----------------|--------------|------------------------------------------------------|
| `RenderConfig` | `args.rs`    | DPI setting for rasterization.                       |
| `OutputFormat` | `args.rs`    | Enum: `Jpg`, `Png`, `WebP`.                         |
| `InputMode`    | `args.rs`    | Enum: `Single(PathBuf)`, `Batch(Vec<PathBuf>)`.     |
| `render_page`  | `render.rs`  | Rasterizes one PDF page → `DynamicImage`.            |
| `save`         | `encode.rs`  | Encodes `DynamicImage` → file at given path.         |
| `process_pdf`  | `process.rs` | End-to-end pipeline per document.                    |

---

## Output Naming Convention

```
<input-stem>-p<NNN>.<ext>
```

Examples for a 3-page `artwork.pdf`:

```
artwork-p001.jpg
artwork-p002.jpg
artwork-p003.jpg
```

Page numbers are zero-padded to 3 digits, consistent with how prepress batch
workflows sort and process files.

---

## DPI Reference for Prepress

| Use case                         | DPI     | Format     | Notes                              |
|----------------------------------|---------|------------|------------------------------------|
| Screen proof / quick preview     | 72–96   | JPG        | Not for print evaluation           |
| Soft proof / client approval     | 150     | JPG or PNG | **Default**                        |
| Print-ready / prepress           | 300     | JPG        | Use `-prepress` flag               |
| Large format / banner            | 100–150 | PNG        | Resolution depends on viewing distance |

---

## PDF Inspection Tools

Useful for verifying output and debugging render results:

```bash
# Decompress a PDF into human-readable form
qpdf --qdf --object-streams=disable input.pdf readable.pdf

# Show page boxes and dimensions
pdfinfo input.pdf

# Structural integrity check
qpdf --check input.pdf
```

Install: `brew install qpdf poppler` (macOS) or `apt install qpdf poppler-utils`
(Debian/Ubuntu).

---

## Benchmarks

Measured on production prepress PDFs (CMYK, image-heavy). Hardware: Apple Mac
Studio (Apple Silicon). Binary built with `cargo install --path .` (release).

Acrobat settings: Save As JPEG, Quality: Maximum, Baseline (Standard),
Colorspace: CMYK, Color Management: Embed profile. Output quality between
p2i and Acrobat at equivalent DPI is visually indistinguishable for prepress
flattening workflows.

**Fixture A — 2-page, object-dense (p2i v0.4.0)**

| DPI  | p2i       | Acrobat Pro | Speedup   |
|------|-----------|-------------|-----------|
| 150  | 878ms     | —           | —         |
| 300  | 1.64s     | 6s          | ~3.6×     |
| 600  | 4.19s     | 1m 46s      | ~25×      |

**Fixture B — 52-page book, medium-density objects (p2i v0.4.0)**

| DPI  | p2i       | Acrobat Pro | Speedup   |
|------|-----------|-------------|-----------|
| 150  | 2.88s     | —           | —         |
| 300  | 5.70s     | 18s         | ~3.2×     |
| 600  | 16.46s    | 2m 3s       | ~7.5×     |

> **Note:** The speedup gap narrows at higher page counts because Acrobat's
> fixed startup and color management overhead is proportionally smaller across
> more pages. The render-time advantage remains significant at all tested DPIs.

> **2400 DPI:** PDFium hits an internal bitmap allocation ceiling at this
> resolution on standard page sizes (~540 megapixels / ~1.6 GB raw per page).
> Not a supported target; 600 DPI is the practical maximum.

---

## Known Limitations

### Native library dependency

PDFium is not a pure-Rust dependency. The `pdfium-render` crate requires
`libpdfium` to be present at runtime. Prebuilt binaries are available from
[pdfium-binaries](https://github.com/bblanchon/pdfium-binaries), but this
does require one manual setup step that `ptrim` and `prsz` do not.

### No TrimBox-aware cropping

Rasterization uses the CropBox/MediaBox rendering boundary, not the TrimBox.
Marks and bleed outside the TrimBox appear in the output if the PDF has a
MediaBox larger than the TrimBox. The intended workflow is:

```
ptrim → prsz -t → p2i
```

to strip marks, crop to trim, then rasterize.

### Batch performance scales O(n) per file

Because rendering is single-threaded (see above), batch jobs grow linearly with
total page count across all inputs. A 20-file batch of 10-page documents renders
200 pages serially. There is no intra-document or inter-document parallelism
until the PDFium C library exposes thread-safe document access. For time-sensitive
batch workflows, consider splitting the input list across multiple `p2i` invocations
in parallel shell processes as a workaround.

### JPEG quality is fixed

JPEG encoding quality is not currently configurable. A `-quality <0–100>` flag
is a natural future addition.

### Spot color rendering

PDFium renders spot colors as CMYK approximations. Output is not
separation-accurate for spot or Pantone inks. For color-accurate proofing of
spot channels, a RIP is required.

---

## Composability with the Series

The three tools compose naturally as a prepress pipeline:

```
pdf-mark-removal (ptrim)           → strip all content outside TrimBox
resize_to_bleed_or_trim_pdf (prsz) → resize MediaBox/CropBox to TrimBox
pdf-to-image (p2i)                 → rasterize to JPG/PNG/WebP
```

Each tool is independently useful. Composition is by convention (output of one
piped as input to the next), not by code coupling.

---

## A Note on rustybara

The shared types across all three repos (`Rect`, `Matrix`, page box readers) are
candidates for eventual extraction into a common prepress-focused Rust crate
(`rustybara`). That effort is future work and does not affect the design or
implementation of this repository. `p2i` is and remains a standalone tool.

---

## Contributing

Contributions are welcome. When making changes:

- Run the full test suite (`cargo test`) before submitting a pull request.
- Do not modify the PDFs passed as input — open read-only.
- DPI conversion must use the `pts / 72.0 * dpi` formula consistently.
- The `-prepress` flag resolves to 300 DPI + JPG.

---

## References

### PDF specification

- **PDF Reference 1.7** (ISO 32000-1:2008) —
  [Adobe PDF Reference, Sixth Edition](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf).
  Chapter 14.11.2 (Page Boundaries), Chapter 8 (Graphics, coordinate systems).

### Crate documentation

- **pdfium-render** — [docs.rs/pdfium-render](https://docs.rs/pdfium-render)
- **image** — [docs.rs/image](https://docs.rs/image)
- **rayon** — [docs.rs/rayon](https://docs.rs/rayon)

### PDFium native binaries

- [github.com/bblanchon/pdfium-binaries](https://github.com/bblanchon/pdfium-binaries)

### Tools

- **qpdf** — [qpdf.sourceforge.io](https://qpdf.sourceforge.io/)
- **Poppler utilities** — [poppler.freedesktop.org](https://poppler.freedesktop.org/)

---

## License

MIT — consistent with the sibling repositories.

Copyright (c) 2025 Addy Alvarado
