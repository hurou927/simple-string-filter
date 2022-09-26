

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub file: Option<String>,

    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    #[clap(short, long, value_parser, default_value_t = false)]
    pub raw: bool,

    #[clap(short = 'e', long, value_parser, default_value_t = false)]
    pub json_encode: bool,

    #[clap(short = 'd', long, value_parser, default_value_t = false)]
    pub json_decode: bool,

    #[clap(short = 'u', long, value_parser, default_value_t = false)]
    pub lf: bool,

    #[clap(short = 'w', long, value_parser, default_value_t = false)]
    pub crlf: bool,
}
