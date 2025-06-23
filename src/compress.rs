use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use anyhow::{Context, Result};
use zstd::stream::Encoder;

/// List of file extensions to skip compression (already compressed formats)
const SKIP_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".gif", ".mp4", ".mp3", ".zip", ".gz", ".bz2", ".xz", ".7z", ".rar", ".ogg", ".webp", ".pdf"];

fn should_skip_compression(path: &str) -> bool {
    SKIP_EXTENSIONS.iter().any(|ext| path.to_lowercase().ends_with(ext))
}

/// Compress a single file using zstd, streaming from disk
pub fn compress_file(input: &str, output: &str, level: i32) -> Result<(u64, u64)> {
    if should_skip_compression(input) {
        anyhow::bail!("File type is already compressed, skipping: {}", input);
    }
    let infile = File::open(input).with_context(|| format!("Failed to open input file: {}", input))?;
    let mut reader = BufReader::new(infile);
    let outfile = File::create(output).with_context(|| format!("Failed to create output file: {}", output))?;
    let mut encoder = Encoder::new(BufWriter::new(outfile), level)?;
    let orig_size = io::copy(&mut reader, &mut encoder)?;
    let mut enc = encoder.finish()?;
    enc.flush()?;
    let comp_size = fs::metadata(output)?.len();
    Ok((orig_size, comp_size))
}
