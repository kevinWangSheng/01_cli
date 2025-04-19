use anyhow::Result;
use clap::Parser;
use process::OutputFormat;
use std::fs;
mod process;
fn main() -> Result<(), anyhow::Error> {
    let args = process::Opts::parse();
    let cmd = args.cmd;
    match cmd {
        process::SubCommand::Csv(cmd) => {
            // parser the csv input
            let file = fs::File::open(cmd.input)?;
            let mut csv_reader = csv::Reader::from_reader(file);
            let csv_data = process::read_csv_data(&mut csv_reader)?;
            // write to json file
            let format = cmd.format.parse::<OutputFormat>()?;
            let content = match format {
                OutputFormat::Json => serde_json::to_string_pretty(&csv_data)?,
                OutputFormat::Yaml => serde_yaml::to_string(&csv_data)?,
                OutputFormat::Toml => process::process_toml(&csv_data, cmd.toml_root_key)?,
            };
            fs::write(cmd.output, content)?;
        }
    }

    Ok(())
}
