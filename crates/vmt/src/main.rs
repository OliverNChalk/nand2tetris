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
    for (line, source, res) in opcodes {
        let hack = match res {
            Ok(hack) => hack,
            Err(err) => {
                println!("ERR: {err}");
                continue;
            }
        };

        println!("// L{line}: {source}");
        for ix in hack.bytecode(&mut label_counter) {
            println!("{ix}");
        }
    }

    Ok(())
}
