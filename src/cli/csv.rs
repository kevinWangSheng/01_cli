use std::collections::HashMap;
use std::str::FromStr;

use clap::ArgAction::SetFalse;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::utils::{verify_file_exists, verify_format_valid};
#[derive(Parser, Debug)]
pub struct CsvOpts {
    // this value were use like "input.csv".into() to convert it into a String
    #[arg(short,long,default_value="input.csv",value_parser=verify_file_exists)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(long,default_value="json",value_parser=verify_format_valid)]
    pub format: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,
    #[arg(long,action=SetFalse,default_value_t=true)]
    pub header: bool,
    #[arg(short, long, default_value_t = true)]
    pub verbose: bool,

    #[arg(long, default_value = "items")]
    pub toml_root_key: String,
}

pub enum OutputFormat {
    Yaml,
    Json,
    Toml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" => Ok(OutputFormat::Yaml),
            "json" => Ok(OutputFormat::Json),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "first_name")]
    first_name: String,
    #[serde(rename = "last_name")]
    last_name: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "gender")]
    gender: String,
    #[serde(rename = "ip_address")]
    ip_address: String,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "avatar")]
    avatar: String,
    #[serde(rename = "car")]
    car: String,
}

#[derive(Serialize)]
pub struct TomlRoot {
    #[serde(flatten)]
    pub data: HashMap<String, Vec<HashMap<String, String>>>,
}
