#[macro_use]
extern crate clap;
extern crate encoding;

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::{self, Read};

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

fn iso_8859_1_to_utf_8(input: &[u8]) -> Result<String, String> {
    Ok(ISO_8859_1.decode(input, DecoderTrap::Strict)?)
}

fn main() {
    // This takes care of --help and --version, but `env::args()` is still
    // what drives the application.
    let _ = clap_app!(converter =>
        (version: "0.1-dev")
        (author: "Mia Boulay <mia.boulay@linux.com>")
        (about: "Convert ISO-8859-1 SOURCE to UTF-8 DEST")
        (usage: "converter [FLAGS] [SOURCE] [DEST]")
        (@arg SOURCE: "STDIN or FILE")
        (@arg DEST: "STDOUT or FILE")
        (after_help: "EXAMPLES:
    converter < latin1.txt > utf8.xml
    converter latin1.txt utf8.xml
    converter latin1.txt                # Output to STDOUT
    converter                           # CTRL+D to exit")
    ).get_matches();

    let args: Vec<String> = env::args().collect();
    let mut config = Config::new(&args).unwrap();

    // Reading and converting BUFSIZE bytes at a time
    let mut bytes = vec![0u8; BUFSIZE];
    while {
        let count = config.source.read(bytes.as_mut_slice()).unwrap();
        config.dest.write(
            iso_8859_1_to_utf_8(&bytes[0..count]).unwrap().as_bytes()
        ).unwrap();
        count != 0
    } {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decoding() {
        // Letter é (different in ISO-8859-1 and UTF-8)
        assert_eq!(iso_8859_1_to_utf_8(&[0xe9]).unwrap().as_bytes(), [0xc3, 0xa9]);

        // Letter 'a'
        assert_eq!(iso_8859_1_to_utf_8(&[0x61]).unwrap(), "a");

        // Empty
        assert_eq!(iso_8859_1_to_utf_8(&[]).unwrap(), "");
    }
}
