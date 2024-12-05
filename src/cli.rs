use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(rename_all = "kebab-case")]
pub struct AocCli {
    #[arg(short, long)]
    pub day: u8,
    #[arg(short, long, default_value = None)]
    pub input: Option<PathBuf>,
}
