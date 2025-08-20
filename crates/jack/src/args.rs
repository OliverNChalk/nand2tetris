use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Args {
    /// Either a single Jack file or a directory containing Jack files.
    pub(crate) path: PathBuf,
}
