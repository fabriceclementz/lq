use std::io::{stdin, BufReader};
use std::str::FromStr;

use clap::Parser;
use lq::decoder::Decoder;
use lq::error::Result;
use lq::input::{InputFormat, LineReader};
use lq::transformer::{TransformFormat, Transformer};

#[derive(Parser, Debug)]
struct Cli {
    /// Input format to process
    input_format: String,

    /// Transform format
    #[arg(short('c'), long)]
    transform_format: String,

    #[arg(short, long)]
    verbose: Option<bool>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let verbose = args.verbose.unwrap_or(false);

    let reader = BufReader::new(stdin());
    let decoder = Decoder::new(
        Box::new(LineReader::new(reader)),
        InputFormat::from_str(&args.input_format)?,
    );

    let mut transformer = Transformer::new(
        Box::new(decoder),
        TransformFormat::from_str(&args.transform_format)?,
    );

    loop {
        match transformer.next() {
            Some(Ok(transformed_line)) => {
                println!(
                    "{}",
                    String::from_utf8(transformed_line).expect("only UTF-8 is supported")
                );
            }
            Some(Err(e)) => {
                if verbose {
                    eprintln!("{}", e);
                }
                break;
            }
            None => break,
        }
    }

    Ok(())
}
