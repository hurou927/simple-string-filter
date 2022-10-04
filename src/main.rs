mod command;
mod core;
mod filter;

use crate::core::error::FilterError;
use crate::core::processor;
use clap::Parser;
use command::{Args, FilterCommand};
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};

fn run<R, W>(reader: R, writer: W, filter_command: &FilterCommand) -> Result<(), FilterError>
where
    R: BufRead,
    W: Write,
{
    match filter_command {
        FilterCommand::JsonEcoder(filter) => {
            processor::run_all_input_as_utf8(reader, writer, |buffer| filter.run(buffer))
        }

        FilterCommand::JsonDecoder(filter) => {
            processor::run_all_input_as_utf8(reader, writer, |buffer| filter.run(buffer))
        }

        FilterCommand::ToLf(filter) => {
            processor::run_per_line_as_byte(reader, writer, |buffer| filter.run(buffer))
        }

        FilterCommand::ToCrLf(filter) => {
            processor::run_per_line_as_byte(reader, writer, |buffer| filter.run(buffer))
        }

        FilterCommand::DoNothing => processor::do_nothing(reader, writer),
    }
}

fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(ref filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let writer: Box<dyn Write> = match args.output {
        None => Box::new(BufWriter::new(stdout().lock())),
        Some(ref filename) => Box::new(BufWriter::new(fs::File::create(filename).unwrap())),
    };

    let filter = FilterCommand::from(args);

    run(reader, writer, &filter).unwrap();
}
