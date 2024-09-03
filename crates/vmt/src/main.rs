mod opts;
mod vm;

fn main() -> eyre::Result<()> {
    use clap::Parser;
    use eyre::eyre;
    use shared::hack;
    use vm::Element;

    // Parse command line args.
    let opts = opts::Opts::parse();

    // Load file & parse all lines.
    let input = std::fs::read_to_string(opts.file)?;
    let parse_results = input
        .lines()
        .map(|line| {
            // Strip everything after the comment.
            line.trim()
                .split_once("//")
                .map(|(left, _)| left)
                .unwrap_or(line)
        })
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(number, source)| (number + 1, source.to_owned(), source.parse::<Element>()));

    // First pass, extract all labels and check for any errors.
    let mut elements = Vec::default();
    let mut hack_labels = hack::Labels::default();
    let mut errors = false;
    for (number, source, res) in parse_results {
        let element = match res {
            Ok(element) => element,
            Err(err) => {
                eprintln!("ERR: {err}\n{source}");
                errors = true;

                continue;
            }
        };

        // Push the element.
        elements.push((number, source, element));
    }

    // Bail if we had errors in the first pass.
    if errors {
        return Err(eyre!("First pass failed, see stderr for more information"));
    }

    // Generate hack assembly for all parsed lines.
    for (line, source, element) in elements {
        println!("// L{line}: {source}");
        for ix in element.bytecode(&mut hack_labels) {
            println!("{ix}");
        }
    }

    Ok(())
}
