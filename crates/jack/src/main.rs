use std::io::{BufWriter, Write};

use crate::args::Action;
use crate::parser::parse;
use crate::tokenizer::Tokenizer;

mod args;
mod parser;
mod tokenizer;

fn main() {
    use clap::Parser;

    // Parse command line args.
    let args = args::Args::parse();

    // Read the source file into memory.
    let source = std::fs::read_to_string(&args.path).unwrap();

    // Tokenize the source file.
    let tokenizer = Tokenizer::new(&source);

    // Execute requested action.
    match args.action {
        Action::Tokenize => {
            let stdout = std::io::stdout().lock();
            let mut output = BufWriter::new(stdout);
            writeln!(output, "<tokens>").unwrap();
            for token in tokenizer {
                token.unwrap().write_xml(&mut output);
                writeln!(output).unwrap();
            }
            writeln!(output, "</tokens>").unwrap();
        }
        Action::Parse => {
            let class = parse(tokenizer);

            println!("{class:#?}")
        }
    }
}
