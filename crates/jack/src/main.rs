use std::io::{BufWriter, Write};

use crate::tokenizer::Tokenizer;

mod args;
mod tokenizer;

fn main() {
    use clap::Parser;

    // Parse command line args.
    let args = args::Args::parse();

    // Read the source file into memory.
    let source = std::fs::read_to_string(&args.path).unwrap();

    // Tokenize the source file.
    let tokenizer = Tokenizer::new(&source);
    let stdout = std::io::stdout().lock();
    let mut output = BufWriter::new(stdout);
    writeln!(output, "<tokens>").unwrap();
    for token in tokenizer {
        token.unwrap().write_xml(&mut output);
        writeln!(output).unwrap();
    }
    writeln!(output, "</tokens>").unwrap();
}
