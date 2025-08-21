use std::process::ExitCode;

mod args;
mod parser;
mod tokenizer;

fn main() -> ExitCode {
    use std::io::{BufWriter, Write};

    use clap::Parser as _;

    use crate::args::Action;
    use crate::parser::Parser;
    use crate::tokenizer::Tokenizer;

    // Parse command line args.
    let args = args::Args::parse();

    // Read the source file into memory.
    let source = std::fs::read_to_string(&args.path).unwrap();

    // Tokenize the source file.
    let mut tokenizer = Tokenizer::new(&source);

    // Execute requested action.
    match args.action {
        Action::Tokenize => {
            let stdout = std::io::stdout().lock();
            let mut output = BufWriter::new(stdout);
            writeln!(output, "<tokens>").unwrap();
            while let Some(token) = tokenizer.next() {
                token.unwrap().write_xml(&mut output);
                writeln!(output).unwrap();
            }
            writeln!(output, "</tokens>").unwrap();
        }
        Action::Parse => match Parser::parse(&mut tokenizer) {
            Ok(class) => println!("{class:#?}"),
            Err(err) => {
                eprintln!("Failed to parse provided source file, the next two unparsed lines are:");
                for line in tokenizer.remaining().lines().take(3) {
                    eprintln!("==> {line}");
                }
                eprintln!("\nError: {err}");

                return ExitCode::FAILURE;
            }
        },
    }

    ExitCode::SUCCESS
}
