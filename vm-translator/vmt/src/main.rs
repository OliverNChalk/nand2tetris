mod opts;
mod parsed_file;
mod vm;

use clap::Parser;
use parsed_file::ParsedFile;

fn main() -> anyhow::Result<()> {
    // Parse command line args.
    let opts = opts::Opts::parse();

    // Load file & parse all lines.
    let parsed = ParsedFile::from_file(&opts.file);

    println!("{parsed:#?}");

    Ok(())
}
