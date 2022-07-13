use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};

fn main() -> io::Result<()> {
    const UBER: u8 = CompressionLevel::UberCompression as u8;
    let mut file = input()?;
    let mut bs = vec![];
    file.read_to_end(&mut bs)?;
    let compressed = compress_to_vec_zlib(&bs, UBER);
    io::stdout()
        .write_all(&compressed)
        .expect("wrote compressed data");

    Ok(())
}

fn input() -> io::Result<Box<dyn io::Read>> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => Ok(Box::new(io::stdin())),
        1 => Ok(Box::new(fs::File::open(&args[0])?)),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("too many arguments: {args:?}"),
        )),
    }
}
