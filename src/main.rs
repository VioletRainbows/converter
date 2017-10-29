extern crate encoding;

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

const BUFSIZE: usize = 4096;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} SOURCE DEST", args[0]);
        eprintln!("Convert ISO-8859-1 SOURCE to UTF-8 DEST");
        process::exit(1);
    }

    // File for reading
    let pread = Path::new(&args[1]);
    let mut fread = File::open(&pread).unwrap();

    // File for writing
    let pwrite = Path::new(&args[2]);
    let mut fwrite = File::create(&pwrite).unwrap();

    // Reading and converting BUFSIZE bytes at a time
    let mut bytes = vec![0u8; BUFSIZE];
    while {
        let count = fread.read(bytes.as_mut_slice()).unwrap();
        fwrite.write(
            // Convert from ISO-8859-1 (latin-1) to UTF-8
            ISO_8859_1
                .decode(&mut bytes[0..count], DecoderTrap::Strict)
                .unwrap()
                .as_bytes(),
        ).unwrap();
        count == BUFSIZE
    } {}
}
