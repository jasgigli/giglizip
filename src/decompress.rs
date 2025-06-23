use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use anyhow::{Context, Result};
use zstd::stream::Decoder;

/// Decompress a single file using zstd, streaming from disk
pub fn decompress_file(input: &str, output: &str) -> Result<(u64, u64)> {
    let infile = File::open(input).with_context(|| format!("Failed to open input file: {}", input))?;
    let comp_size = infile.metadata()?.len();
    let mut decoder = Decoder::new(BufReader::new(infile))?;
    let outfile = File::create(output).with_context(|| format!("Failed to create output file: {}", output))?;
    let mut writer = BufWriter::new(outfile);
    let orig_size = io::copy(&mut decoder, &mut writer)?;
    writer.flush()?;
    Ok((comp_size, orig_size))
}
