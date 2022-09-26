mod command;

use command::Args;
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Read, Write};

use clap::Parser;

fn run<F>(mut reader: Box<dyn BufRead>, mut writer: Box<dyn Write>, f: F)
where F: FnOnce(&str) -> String
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    let out = f(&buffer);

    writer.write_all(out.as_bytes()).unwrap();
}


fn main() {
    let args = Args::parse();

    let mut reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let mut writer: Box<dyn Write> = match args.output {
        None => Box::new(BufWriter::new(stdout().lock())),
        Some(filename) => Box::new(BufWriter::new(fs::File::create(filename).unwrap()))
    };

    // https://www.rfc-editor.org/rfc/rfc8259.html
    if args.json_decode {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        if args.raw {
            buffer = "\"".to_owned() + &buffer + "\"";
        }

        let json_decoded_string: String = serde_json::from_str(&buffer).unwrap();
        writer.write_all(json_decoded_string.as_bytes()).unwrap();
    } else if args.json_encode {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();

        let mut json_encoded_string: String = serde_json::to_string_pretty(&buffer).unwrap();
        if args.raw {
            // remove first and last double-quote
            json_encoded_string.pop();
            json_encoded_string.remove(0);
        }
        writer.write_all(json_encoded_string.as_bytes()).unwrap();
    } else {
        println!("do-nothing");
    }
}
