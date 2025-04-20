use anyhow::Result;
use clap::Parser;
use process::{Opts, SubCommand, csv_generate::*};
use std::fs;
use template::process::password_generate::password_gen;
use zxcvbn::zxcvbn;
mod process;
fn main() -> Result<(), anyhow::Error> {
    let args = Opts::parse();
    let cmd = args.cmd;
    match cmd {
        SubCommand::Csv(cmd) => {
            // parser the csv input
            let file = fs::File::open(cmd.input)?;
            let mut csv_reader = csv::Reader::from_reader(file);
            let csv_data = read_csv_data(&mut csv_reader)?;
            // write to json file
            let format = cmd.format.parse::<OutputFormat>()?;
            let content = match format {
                OutputFormat::Json => serde_json::to_string_pretty(&csv_data)?,
                OutputFormat::Yaml => serde_yaml::to_string(&csv_data)?,
                OutputFormat::Toml => process_toml(&csv_data, cmd.toml_root_key)?,
            };
            fs::write(cmd.output, content)?;
        }
        SubCommand::Password(cmd) => {
            let password = password_gen(
                cmd.length,
                cmd.uppercase,
                cmd.lowercase,
                cmd.digits,
                cmd.symbols,
            )?;
            eprintln!("the password is :{}", password);
            eprintln!(
                "the password score is :{:?}",
                zxcvbn(&password, &[]).score()
            );
        }
    }

    Ok(())
}
