use clap::Parser;

use crate::filter::{
    json::{JsonDecodeFilter, JsonEncodeFilter},
    new_line::{ToCrLfFilter, ToLfFilter},
};

/// Simple string filter
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub file: Option<String>,

    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    #[clap(long, value_parser, default_value_t = false)]
    pub force: bool,

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

#[derive(Debug)]
pub enum FilterCommand {
    JsonEcoder(JsonEncodeFilter),
    JsonDecoder(JsonDecodeFilter),
    ToLf(ToLfFilter),
    ToCrLf(ToCrLfFilter),
    DoNothing,
}

impl From<&Args> for FilterCommand {
    fn from(arg: &Args) -> Self {
        if arg.json_encode {
            let filter = JsonEncodeFilter::new(arg.raw);
            FilterCommand::JsonEcoder(filter)
        } else if arg.json_decode {
            let filter = JsonDecodeFilter::new(arg.raw);
            FilterCommand::JsonDecoder(filter)
        } else if arg.lf {
            let filter = ToLfFilter {};
            FilterCommand::ToLf(filter)
        } else if arg.crlf {
            let filter = ToCrLfFilter {};
            FilterCommand::ToCrLf(filter)
        } else {
            FilterCommand::DoNothing
        }
    }
}
