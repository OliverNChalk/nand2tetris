use crate::tokenizer::Tokenizer;

mod args;
mod tokenizer;

fn main() {
    use clap::Parser;

    // Parse command line args.
    let args = args::Args::parse();

    // Read the source file into memory.
    let source = std::fs::read_to_string(&args.path).unwrap();

    // Create a tokenizer.
    let tokenizer = Tokenizer::new(&source);

    for token in tokenizer {
        println!("TOKEN: {:?}", token.unwrap());
    }
}
