mod command;
mod helper;

use command::Args;
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};

use clap::Parser;
use helper::*;


fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let writer: Box<dyn Write> = match args.output {
        None => Box::new(BufWriter::new(stdout().lock())),
        Some(filename) => Box::new(BufWriter::new(fs::File::create(filename).unwrap())),
    };

    if args.json_decode {
        run_all_input(reader, writer, |buffer| {
            // https://www.rfc-editor.org/rfc/rfc8259.html
            if args.raw {
                let new_buf = "\"".to_owned() + buffer + "\"";
                serde_json::from_str(&new_buf).unwrap()
            } else {
                serde_json::from_str(buffer).unwrap()
            }
        });
    } else if args.json_encode {
        run_all_input(reader, writer, |buffer| {
            // https://www.rfc-editor.org/rfc/rfc8259.html
            let mut json_encoded_string = serde_json::to_string_pretty(buffer).unwrap();
            if args.raw {
                // remove first and last double-quote
                json_encoded_string.pop();
                json_encoded_string.remove(0);
            }
            json_encoded_string
        });
    } else if args.lf {
        run_per_line_as_byte(reader, writer, |buffer| {
            let mut new_buffer = buffer.to_owned();
            if new_buffer.ends_with(&[b'\n']) {
                new_buffer.pop();
                if new_buffer.ends_with(&[b'\r']) {
                    new_buffer.pop();
                }
            }
            new_buffer.push(b'\n');
            new_buffer
        });
    } else if args.crlf {
        run_per_line_as_byte(reader, writer, |buffer| {
            let mut new_buffer = buffer.to_owned();
            if new_buffer.ends_with(&[b'\n']) {
                new_buffer.pop();
                if new_buffer.ends_with(&[b'\r']) {
                    new_buffer.pop();
                }
            }
            new_buffer.push(b'\r');
            new_buffer.push(b'\n');
            new_buffer
        });
    } else {
        do_nothing(reader, writer);
    }
}
