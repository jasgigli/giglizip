use clap::Parser;
use std::fs;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    input: String,
    #[arg(long)]
    output: String,
    #[arg(long, default_value_t = 3)]
    level: u8,
    #[arg(long, default_value = "zstd")]
    method: String,
    #[arg(long)]
    decompress: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut input_data = Vec::new();
    fs::File::open(&cli.input)
        .unwrap()
        .read_to_end(&mut input_data)
        .unwrap();
    let output_data = if cli.decompress {
        giglizip_compress::decompress(&input_data)
    } else {
        giglizip_compress::compress(&input_data)
    };
    let mut out = fs::File::create(&cli.output).unwrap();
    out.write_all(&output_data).unwrap();
}
