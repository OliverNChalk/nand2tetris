mod args;
mod opcode;
mod parser;
mod region;

// TODO
//
// 1. Support loading directories.
// 2. Parse all files into VmFile struct that contains the file name and list of
//    opcodes.
// 3. Translate each opcode into assembly.

fn main() -> eyre::Result<()> {
    use clap::Parser;

    use crate::opcode::Counter;
    use crate::parser::VmFile;

    // Parse command line args.
    let args = args::Args::parse();

    // Load & parse all provided files.
    let files = match args.path.is_dir() {
        true => std::fs::read_dir(&args.path)
            .unwrap()
            .map(|res| VmFile::parse_file(res.unwrap().path()))
            .collect(),
        false => vec![VmFile::parse_file(args.path)],
    };

    // Generate hack assembly for all parsed lines.
    let mut label_counter = Counter::default();
    for (_, (line, source, res)) in files
        .iter()
        .flat_map(|file| file.opcodes.iter().map(|opcode| (&file.path, opcode)))
    {
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
