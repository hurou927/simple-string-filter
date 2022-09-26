
use std::fs;
use std::io::{self, BufRead, BufReader, Read, BufWriter, Write, stdout};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: Option<String>,

    #[clap(short, long, value_parser, default_value_t = false)]
    raw: bool,

    #[clap(short = 'e', long, value_parser, default_value_t = false)]
    json_encode: bool,

    #[clap(short = 'd', long, value_parser, default_value_t = false)]
    json_decode: bool,
}

fn main() {
    let args = Args::parse();

    let mut reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    if args.json_decode {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        if args.raw {
            buffer = "\"".to_owned() + &buffer + "\"";
        }

        let json_decoded_string: String = serde_json::from_str(&buffer).unwrap();
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        out.write_all(json_decoded_string.as_bytes()).unwrap();

    } else if args.json_encode {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();

        let mut json_encoded_string: String = serde_json::to_string_pretty(&buffer).unwrap();
        if args.raw {
            // remove first and last double-quote
            json_encoded_string.pop();
            json_encoded_string.remove(0);
        }
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        out.write_all(json_encoded_string.as_bytes()).unwrap();
    } else {
        println!("do-nothing");
    }
}
