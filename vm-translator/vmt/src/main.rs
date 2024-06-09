mod opts;
mod parsed_file;
mod vm;

use clap::Parser;
use parsed_file::ParsedFile;

fn main() -> anyhow::Result<()> {
    // Parse command line args.
    let opts = opts::Opts::parse();

    // Load file & parse all lines.
    let parsed = ParsedFile::from_file(&opts.file)?;

    // Generate hack assembly for all parsed lines.
    for (line, res) in parsed.source {
        let hack = match res {
            Ok(hack) => hack,
            Err(err) => {
                println!("ERR: {err}");
                continue;
            }
        };

        println!("{line}:");
        for line in hack.bytecode() {
            println!("{}", line);
        }
    }

    Ok(())
}
