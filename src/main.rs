// Usage: converter [SOURCE] [DEST]
// Convert ISO-8859-1 SOURCE to UTF-8 DEST
//
// SOURCE: STDIN or FILE
// DEST: STDOUT or FILE
//
// Examples:
// converter < latin1.txt > utf8.xml
// converter latin1.txt utf8.xml
// converter latin1.txt                # Output to STDOUT
// converter                           # CTRL+D to exit

extern crate encoding;

use std::io::{self, Read};

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

const BUFSIZE: usize = 8192;

struct Config<R: io::Read, W: io::Write> {
    source: R,
    dest: W,
}

impl Config<Box<io::Read>, Box<io::Write>> {
    fn new(args: &[String]) -> Result<Self, Box<Error>> {
        if args.len() == 1 {
            Ok(Config {
                source: Box::new(io::stdin()),
                dest: Box::new(io::stdout()),
            })
        } else if args.len() == 2 {
            Ok(Config {
                source: Box::new(File::open(args[1].clone())?),
                dest: Box::new(io::stdout()),
            })
        } else {
            Ok(Config {
                source: Box::new(File::open(args[1].clone())?),
                dest: Box::new(File::create(args[2].clone())?),
            })
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new(&args).unwrap();

    // Reading and converting BUFSIZE bytes at a time
    let mut bytes = vec![0u8; BUFSIZE];
    while {
        let count = config.source.read(bytes.as_mut_slice()).unwrap();
        config.dest.write(
             // Convert from ISO-8859-1 (latin-1) to UTF-8
             ISO_8859_1
                .decode(&mut bytes[0..count], DecoderTrap::Strict)
                .unwrap()
                .as_bytes(),
        ).unwrap();
        count != 0
    } {}
}
