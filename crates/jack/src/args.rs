use std::path::PathBuf;
use std::str::FromStr;

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::{Parser, Subcommand};
use strum::{EnumString, VariantNames};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Args {
    /// Either a single Jack file or a directory containing Jack files.
    pub(crate) path: PathBuf,
    /// The action to perform on the specified path.
    #[arg(
        value_parser = PossibleValuesParser::new(Action::VARIANTS)
            .map(|s| Action::from_str(&s).unwrap())
    )]
    pub(crate) action: Action,
}

#[derive(Debug, Clone, EnumString, strum::VariantNames)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Action {
    Tokenize,
    Parse,
}
