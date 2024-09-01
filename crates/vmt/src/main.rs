mod opts;
mod vm;

use clap::Parser;
use vm::OpCode;

fn main() -> eyre::Result<()> {
    // Parse command line args.
    let opts = opts::Opts::parse();

    // Load file & parse all lines.
    let input = std::fs::read_to_string(opts.file)?;
    let opcodes = input
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .filter(|(_, line)| !line.is_empty() && !line.starts_with("//"))
        .map(|(number, source)| (number + 1, source.to_owned(), source.parse::<OpCode>()));

    // Generate hack assembly for all parsed lines.
    let mut label_counter = vm::Counter::default();
    let mut output = Vec::default();
    let mut errors = false;
    for (line, source, res) in opcodes {
        let hack = match res {
            Ok(hack) => hack,
            Err(err) => {
                eprintln!("ERR: {err}");
                errors = true;
                continue;
            }
        };

        output.push(format!("// L{line}: {source}"));
        for ix in hack.bytecode(&mut label_counter) {
            output.push(format!("{ix}"));
        }
    }

    // Only print the file output if we had no errors.
    if !errors {
        for line in output {
            println!("{line}");
        }
    }

    Ok(())
}
