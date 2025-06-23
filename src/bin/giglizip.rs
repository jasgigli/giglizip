use anyhow::Result;
use clap::Parser;
use dialoguer::{Input, Select};
use std::io::{self, Write};
use textwrap; // This will now work because of the change in Cargo.toml

use giglizip::compress;
use giglizip::decompress;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Compress a file or folder
    Compress {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long, default_value_t = 3)]
        level: i32,
    },
    /// Decompress a .ggz file
    Decompress {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let is_interactive = cli.command.is_none();

    if is_interactive {
        run_interactive_mode()?;
    } else {
        run_cli_mode(cli)?;
    }

    Ok(())
}

fn run_interactive_mode() -> Result<()> {
    loop {
        let action = Select::new()
            .with_prompt("What would you like to do?")
            .items(&["Compress", "Decompress", "Exit"])
            .default(0)
            .interact()?;

        match action {
            0 => handle_interactive_op(false)?, // Compress
            1 => handle_interactive_op(true)?,  // Decompress
            2 => {
                println!("Goodbye!");
                return Ok(());
            }
            _ => unreachable!(),
        }

        println!("\nPress Enter to return to the main menu...");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy)?;
        // Clears the screen for a better UX on the next loop iteration
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush()?;
    }
}

fn handle_interactive_op(is_decompress: bool) -> Result<()> {
    let prompt_noun = if is_decompress {
        "source file (.ggz)"
    } else {
        "source file or folder"
    };
    let input_path: String = Input::new()
        .with_prompt(format!("Enter the path to the {}", prompt_noun))
        .interact_text()?;

    let default_output = if is_decompress {
        input_path
            .strip_suffix(".ggz")
            .unwrap_or(&input_path)
            .to_string()
    } else {
        format!("{}.ggz", input_path)
    };

    let output_path: String = Input::new()
        .with_prompt("Enter the output path (leave blank for default)")
        .default(default_output.clone())
        .allow_empty(true)
        .interact_text()?;

    let final_output_path = if output_path.trim().is_empty() {
        default_output
    } else {
        output_path
    };

    if is_decompress {
        match decompress::decompress_file(&input_path, &final_output_path) {
            Ok((comp_size, orig_size)) => {
                print_result_box(
                    "Decompression Successful!",
                    &input_path,
                    &final_output_path,
                    comp_size,
                    orig_size,
                );
            }
            Err(e) => print_error_box("Decompression Failed", &e.to_string()),
        }
    } else {
        let level: i32 = Input::new()
            .with_prompt("Enter compression level (1-22 for Zstd)")
            .default(3)
            .interact()?;
        match compress::compress_file(&input_path, &final_output_path, level) {
            Ok((orig_size, comp_size)) => {
                print_result_box(
                    "Compression Successful!",
                    &input_path,
                    &final_output_path,
                    orig_size,
                    comp_size,
                );
            }
            Err(e) => print_error_box("Compression Failed", &e.to_string()),
        }
    }
    Ok(())
}

fn run_cli_mode(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Compress {
            input,
            output,
            level,
        }) => {
            let final_output = output.unwrap_or_else(|| format!("{}.ggz", input));
            match compress::compress_file(&input, &final_output, level) {
                Ok((orig_size, comp_size)) => {
                    print_result_box(
                        "Compression Successful!",
                        &input,
                        &final_output,
                        orig_size,
                        comp_size,
                    );
                }
                Err(e) => {
                    print_error_box("Compression Failed", &e.to_string());
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Decompress { input, output }) => {
            let final_output = output
                .unwrap_or_else(|| input.strip_suffix(".ggz").unwrap_or(&input).to_string());
            match decompress::decompress_file(&input, &final_output) {
                Ok((comp_size, orig_size)) => {
                    print_result_box(
                        "Decompression Successful!",
                        &input,
                        &final_output,
                        comp_size,
                        orig_size,
                    );
                }
                Err(e) => {
                    print_error_box("Decompression Failed", &e.to_string());
                    std::process::exit(1);
                }
            }
        }
        None => {
            // This case should not be reached due to clap's handling, but it's here for safety.
            println!("No command specified. Use 'giglizip --help' for usage.");
        }
    }
    Ok(())
}

fn print_result_box(title: &str, input: &str, output: &str, size1: u64, size2: u64) {
    let (orig_size, comp_size, is_compress) = if title.starts_with("Compress") {
        (size1, size2, true)
    } else {
        (size2, size1, false)
    };

    let ratio = if orig_size > 0 {
        100.0 * (1.0 - (comp_size as f64 / orig_size as f64))
    } else {
        0.0
    };

    let orig_size_label = if is_compress { "Original Size" } else { "Decompressed Size" };
    let comp_size_label = if is_compress { "Compressed Size" } else { "Original Size (from .ggz)" };

    let orig_size_str = format!("{} bytes", orig_size);
    let comp_size_str = format!("{} bytes", comp_size);
    let ratio_str = format!("{:.2}% reduction", ratio);

    println!("\n+{:-^58}+", "");
    println!("| {:^56} |", title);
    println!("+{:-^58}+", "");
    println!("| {:<18}: {:<36} |", "Source", input);
    println!("| {:<18}: {:<36} |", "Destination", output);
    println!("|{:-^58}|", "");
    println!("| {:<18}: {:>36} |", orig_size_label, orig_size_str);
    println!("| {:<18}: {:>36} |", comp_size_label, comp_size_str);
    println!("| {:<18}: {:>36} |", "Efficiency", ratio_str);
    println!("+{:-^58}+", "");
}

fn print_error_box(title: &str, msg: &str) {
    let wrapped_msg = textwrap::wrap(msg, 54);

    println!("\n+{:-^58}+", "");
    println!("| {:^56} |", title.to_uppercase());
    println!("+{:-^58}+", "");
    for line in wrapped_msg {
        println!("|  {:<54}  |", line);
    }
    println!("+{:-^58}+", "");
}
