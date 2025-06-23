use clap::Parser;
use std::fs;
use std::io::{Read, Write};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{Input, Select};
use std::env;

use giglizip::compress;
use giglizip::decompress;

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
    if cli.decompress {
        match decompress::decompress_file(&input, &output) {
            Ok((comp_size, orig_size)) => {
                println!(
                    "Decompressed: {} → {} ({} bytes → {} bytes)",
                    input, output, comp_size, orig_size
                );
            },
            Err(e) => {
                eprintln!("[ERROR] Decompression failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match compress::compress_file(&input, &output, cli.level as i32) {
            Ok((orig_size, comp_size)) => {
                if comp_size >= orig_size {
                    println!("[WARN] Compression ineffective: output is not smaller than input");
                }
                let ratio = if orig_size > 0 {
                    100.0 * (1.0 - (comp_size as f64 / orig_size as f64))
                } else { 0.0 };
                println!(
                    "Original: {} bytes, Compressed: {} bytes ({:.2}% reduction)",
                    orig_size, comp_size, ratio
                );
            },
            Err(e) => {
                eprintln!("[ERROR] Compression failed: {}", e);
                std::process::exit(1);
            }
        }
    }
    Ok(())
}
