use clap::Parser;
use std::fs;
use std::io::{Read, Write};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{Input, Select};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    output: Option<String>,
    #[arg(long, default_value_t = 3)]
    level: u8,
    #[arg(long, default_value = "zstd")]
    method: String,
    #[arg(long)]
    decompress: bool,
}

fn main() -> Result<()> {
    let mut cli = Cli::parse();
    let args: Vec<String> = env::args().collect();
    let interactive = args.len() == 1;
    if interactive {
        // Interactive mode
        let action = Select::new()
            .with_prompt("What would you like to do?")
            .items(&["compress", "decompress"])
            .default(0)
            .interact()?;
        cli.decompress = action == 1;
        cli.input = Some(Input::new().with_prompt("Enter the source file or directory path").interact_text()?);
        let default_output = match &cli.input {
            Some(input) => {
                if cli.decompress && input.ends_with(".ggz") {
                    input.trim_end_matches(".ggz").to_string()
                } else if !cli.decompress && !input.ends_with(".ggz") {
                    format!("{}.ggz", input)
                } else {
                    input.clone()
                }
            },
            None => String::new(),
        };
        let output: String = Input::new()
            .with_prompt("Enter output filename (leave blank to auto-generate)")
            .default(default_output.clone()) // <-- fix: clone here
            .allow_empty(true)
            .interact_text()?;
        cli.output = if output.trim().is_empty() { Some(default_output) } else { Some(output) };
    }
    let input = cli.input.clone().expect("Input is required");
    let output = cli.output.clone().expect("Output is required");
    let metadata = fs::metadata(&input)
        .with_context(|| format!("Failed to stat input file: {}", input))?;
    let pb = ProgressBar::new(metadata.len());
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    let mut input_file = fs::File::open(&input)
        .with_context(|| format!("Failed to open input file: {}", input))?;
    let mut input_data = Vec::with_capacity(metadata.len() as usize);
    let mut buf = [0u8; 8192];
    loop {
        let n = input_file.read(&mut buf)?;
        if n == 0 { break; }
        input_data.extend_from_slice(&buf[..n]);
        pb.inc(n as u64);
    }
    pb.finish_with_message("Read complete");
    let pb2 = ProgressBar::new(input_data.len() as u64);
    pb2.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.magenta/blue}] {bytes}/{total_bytes} ({eta}) compressing...")
        .unwrap()
        .progress_chars("#>-")
    );
    let output_data = if cli.decompress {
        giglizip_compress::decompress(&input_data)
    } else {
        giglizip_compress::compress(&input_data)
    };
    pb2.finish_with_message("Done");
    let mut out = fs::File::create(&output)
        .with_context(|| format!("Failed to create output file: {}", output))?;
    out.write_all(&output_data).context("Failed to write output file")?;
    Ok(())
}
