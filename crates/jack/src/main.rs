use std::sync::atomic::AtomicU64;

mod args;
mod code_gen;
mod parser;
mod tokenizer;

fn main() -> std::process::ExitCode {
    use std::fmt::Display;
    use std::io::{BufWriter, Write};
    use std::process::ExitCode;

    use clap::Parser as _;

    use crate::args::Action;
    use crate::parser::structure::Class;
    use crate::tokenizer::Tokenizer;

    fn print_error(tokenizer: Tokenizer, err: impl Display) {
        eprintln!("Failed to parse provided source file, the next two unparsed lines are:");
        for line in tokenizer.remaining().lines().take(3) {
            eprintln!("==> {line}");
        }
        eprintln!("\nError: {err}");
    }

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
        Action::Parse => match Class::parse(&mut tokenizer) {
            Ok(class) => println!("{class:#?}"),
            Err(err) => {
                print_error(tokenizer, err);

                return ExitCode::FAILURE;
            }
        },
        Action::Compile => {
            let class = match Class::parse(&mut tokenizer) {
                Ok(class) => class,
                Err(err) => {
                    print_error(tokenizer, err);

                    return ExitCode::FAILURE;
                }
            };

            let vm_symbols = Box::leak(Box::new(AtomicU64::new(0)));
            match code_gen::compile(vm_symbols, &class) {
                Ok(code) => {
                    for code in code {
                        println!("{code}");
                    }
                }
                Err(err) => {
                    print_error(tokenizer, err);

                    return ExitCode::FAILURE;
                }
            }
        }
    }

    ExitCode::SUCCESS
}
