mod command;
mod core;
mod filter;

use crate::core::processor;
use clap::Parser;
use command::Args;
use filter::json::{JsonDecoder, JsonEncoder};
use filter::new_line::{ToCrLf, ToLf};
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};

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
        processor::run_all_input_as_utf8(reader, writer, |buffer| jd.json_decode(buffer)).unwrap();
    } else if args.json_encode {
        let je = JsonEncoder::new(args.raw);
        processor::run_all_input_as_utf8(reader, writer, |buffer| je.json_ecode(buffer)).unwrap();
    } else if args.lf {
        let to_lf = ToLf {};
        processor::run_per_line_as_byte(reader, writer, |buffer| to_lf.run(buffer)).unwrap();
    } else if args.crlf {
        let to_cr_lf = ToCrLf {};
        processor::run_per_line_as_byte(reader, writer, |buffer| to_cr_lf.run(buffer)).unwrap();
    } else {
        processor::do_nothing(reader, writer).unwrap();
    }
}
