use giglizip_compress::compress;
use std::env;
use std::fs;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: simple_cli <input> <output>");
        std::process::exit(1);
    }
    let mut input = Vec::new();
    fs::File::open(&args[1])
        .unwrap()
        .read_to_end(&mut input)
        .unwrap();
    let output = compress(&input);
    let mut out = fs::File::create(&args[2]).unwrap();
    out.write_all(&output).unwrap();
}
