pub mod base64;
pub mod csv_generate;
pub mod password_generate;
use base64::Base64Ops;
use clap::Parser;
use csv_generate::CsvOpts;
use password_generate::PasswordOpts;

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
}
