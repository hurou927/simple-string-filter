mod command;
mod core;
mod helper;
mod filter;

use command::Args;
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};
use filter::json::{JsonEncoder, JsonDecoder};
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
        let jd = JsonDecoder::new(args.raw);
        run_all_input_as_utf8(reader, writer, |buffer| jd.json_decode(buffer)).unwrap();

    } else if args.json_encode {
        let je = JsonEncoder::new(args.raw);
        run_all_input_as_utf8(reader, writer, |buffer| je.json_ecode(buffer) ).unwrap();

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
            Ok(new_buffer)
        })
        .unwrap();
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
            Ok(new_buffer)
        })
        .unwrap();
    } else {
        do_nothing(reader, writer).unwrap();
    }
}
