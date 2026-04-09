use pdf_to_image::args::{OutputFormat, RenderConfig};
use pdf_to_image::process_pdf;
use std::path::{Path, PathBuf};
use std::process;

fn parse_batch_args(args: &[String]) -> Option<(Vec<PathBuf>, Option<PathBuf>)> {
    if !args.get(1).map_or(false, |arg| arg.starts_with('[')) {
        return None;
    }

    let mut raw_tokens: Vec<String> = Vec::new();
    let mut closing_idx: Option<usize> = None;

    for (i, arg) in args[1..].iter().enumerate() {
        raw_tokens.push(arg.clone());
        if arg.ends_with(']') {
            closing_idx = Some(i + 1);
            break;
        }
    }

    let closing_idx = closing_idx?;

    let joined = raw_tokens.join(" ");
    let inner = joined.trim_start_matches('[').trim_end_matches(']');

    let inputs: Vec<PathBuf> = inner
        .split(',')
        .map(|s| PathBuf::from(s.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .collect();
    if inputs.is_empty() {
        return None;
    }

    let output_dir = args.get(closing_idx + 1).map(PathBuf::from);

    Some((inputs, output_dir))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--version")) {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        process::exit(0);
    }

    let mut dpi: u32 = 150;
    let mut format = OutputFormat::Jpg;
    let mut positional: Vec<String> = Vec::new();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-prepress" => {
                dpi = 300;
                format = OutputFormat::Jpg;
                i += 1;
            }
            "-mega-prepress" => {
                dpi = 600;
                format = OutputFormat::Jpg;
                i += 1;
            }
            "-dpi" => {
                i += 1;
                dpi = args.get(i).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                    eprintln!("Error: -dpi requires a numeric value");
                    process::exit(1);
                });
                i += 1;
            }
            "-format" => {
                i += 1;
                format = match args.get(i).map(|s| s.as_str()) {
                    Some("tiff") => OutputFormat::Tiff,
                    Some("png") => OutputFormat::Png,
                    Some("jpeg") | Some("jpg") => OutputFormat::Jpg,
                    Some("webp") => OutputFormat::WebP,
                    _ => {
                        eprintln!("Error: -format must be jpg, jpeg, png, or webp");
                        process::exit(1);
                    }
                };
                i += 1;
            }
            other => {
                positional.push(other.to_string());
                i += 1;
            }
        }
    }

    let config = RenderConfig { dpi };

    if let Some((inputs, output_dir)) = parse_batch_args(&args) {
        for input_path in &inputs {
            if !input_path.exists() {
                eprintln!("Error: Input path {} does not exists", input_path.display());
                process::exit(1);
            }
            let out_dir = output_dir
                .clone()
                .unwrap_or_else(|| input_path.parent().unwrap_or(Path::new(".")).to_path_buf());

            if !out_dir.exists() {
                std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
                    eprintln!("Error: Output directory {} does not exists", e);
                    process::exit(1);
                })
            }

            if let Err(e) = process_pdf(input_path, &out_dir, &config, &format) {
                eprintln!("Error: Unable to process {}: {}", input_path.display(), e);
                process::exit(1);
            }
            println!("Done: {} → {}", input_path.display(), out_dir.display());
        }
    } else {
        if positional.is_empty() {
            eprintln!(
                "Usage: p2i [-dpi <n>] [-format <tiff|jpg|jpeg|png|webp>] [-prepress] [-mega-prepress] <input.pdf> [output/dir]"
            );
            eprintln!(
                "       p2i [-dpi <n>] [-format <tiff|jpg|jpeg|png|webp>] [-prepress] [-mega-prepress] [input-1.pdf, input-2.pdf, ...][output/dir]"
            );
            process::exit(1);
        }
        let input_path = Path::new(&positional[0]);
        if !input_path.exists() {
            eprintln!("Error: Input path {} does not exists", input_path.display());
            process::exit(1);
        }

        let out_dir = if positional.len() > 1 {
            PathBuf::from(&positional[1])
        } else {
            input_path.parent().unwrap_or(Path::new(".")).to_path_buf()
        };

        if !out_dir.exists() {
            std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
                eprintln!("Error: Output directory {} does not exists", e);
                process::exit(1);
            });
        }

        if let Err(e) = process_pdf(input_path, &out_dir, &config, &format) {
            eprintln!("Error: Unable to process {}: {}", input_path.display(), e);
            process::exit(1);
        }
        println!("Done: {} → {}", input_path.display(), out_dir.display());
    }
}
