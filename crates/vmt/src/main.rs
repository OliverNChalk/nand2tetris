mod args;
mod opcode;
mod parser;
mod region;
mod writer;

fn main() {
    use clap::Parser;

    use crate::parser::VmFile;
    use crate::writer::Writer;

    // Parse command line args.
    let args = args::Args::parse();

    // Load & parse all provided files.
    let files = match args.path.is_dir() {
        true => std::fs::read_dir(&args.path)
            .unwrap()
            .map(|res| res.unwrap().path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("vm"))
            .map(VmFile::parse_file)
            .collect(),
        false => vec![VmFile::parse_file(args.path)],
    };

    // Setup the code writer.
    let writer = Writer::new(files);

    // Generate hack assembly for all parsed lines.
    writer.write();
}
