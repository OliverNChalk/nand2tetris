use std::path::PathBuf;

use clap::{Parser, Subcommand};
use strum::EnumString;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Args {
    /// Either a single Jack file or a directory containing Jack files.
    pub(crate) path: PathBuf,
    /// The action to perform on the specified path.
    pub(crate) action: Action,
}

#[derive(Debug, Clone, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Action {
    Tokenize,
    Parse,
}
