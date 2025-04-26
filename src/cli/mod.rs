pub mod base64;
pub mod csv;
pub mod password;
pub mod text;

use base64::Base64Ops;
use clap::Parser;
use csv::CsvOpts;
use password::PasswordOpts;
use std::path::{Path, PathBuf};
use text::TextSubcommand;

pub fn verify_path(path: &str) -> Result<PathBuf, anyhow::Error> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err(anyhow::anyhow!("Path does not exist"))
    }
}

#[derive(Parser, Debug)]
#[command(version, about, author, long_about)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV file to other format")]
    Csv(CsvOpts),
    #[command(name = "password-gen", about = "Generate password")]
    Password(PasswordOpts),
    #[command(subcommand)]
    Base64(Base64Ops),
    #[command(subcommand)]
    Text(TextSubcommand),
}
