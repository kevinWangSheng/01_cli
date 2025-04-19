use anyhow::Result;
use clap::Parser;
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
            let mut persons = Vec::with_capacity(128);
            for result in csv_reader.deserialize() {
                let record: process::Person = result?;
                persons.push(record);
            }
            // write to json file
            let json = serde_json::to_string_pretty(&persons)?;
            fs::write(cmd.output, json.as_bytes())?;
        }
    }

    Ok(())
}
