mod command;
mod core;
mod filter;

use crate::core::error::FilterError;
use crate::core::processor;
use clap::Parser;
use command::{Args, FilterCommand};

use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};

fn run_filter<R, W>(reader: R, writer: W, filter_command: &FilterCommand) -> Result<(), FilterError>
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

fn run_main(args: &Args) -> Result<(), FilterError> {
    let reader: Box<dyn BufRead> = match args.file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(ref filename) => {
            let file = std::fs::OpenOptions::new()
                .read(true)
                .open(filename)
                .map_err(|_| FilterError::IoFaileToOpenFile{ filename: filename.to_owned() })?;
            Box::new(BufReader::new(file))
        }
    };

    let writer: Box<dyn Write> = match args.output {
        None => Box::new(BufWriter::new(stdout().lock())),
        Some(ref filename) => {
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(!args.force)
                .create(args.force)
                .open(filename)
                .map_err(|_| FilterError::IoFaileToCreateFile { filename: filename.to_owned() })?;

            Box::new(BufWriter::new(file))
        }
    };

    let filter = FilterCommand::from(args);

    run_filter(reader, writer, &filter)
}

fn main() {
    let args = Args::parse();
    match run_main(&args) {
        Ok(_) => {},
        Err(err) =>{
            eprintln!("Error: {}", err);
            std::process::exit(0x0100);
        }
    };
}
