use anyhow::{Context, Ok};
use clap::{ArgAction::SetFalse, Parser};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Read, str::FromStr};
// parse rcli csv --input.csv --output.json() --delimiter=, --header=true --verbose=true

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

fn verify_file_exists(path: &str) -> Result<String, anyhow::Error> {
    match fs::metadata(path) {
        std::result::Result::Ok(metadata) => {
            // 首先匹配 Ok 变体
            if metadata.is_file() {
                // 如果是文件，返回 Ok 包含路径字符串
                Ok(path.to_string())
            } else {
                // 如果元数据获取成功，但不是文件（例如是目录），返回错误
                Err(anyhow::anyhow!("Path exists but is not a file: {}", path))
            }
        }
        Err(e) => Err(anyhow::anyhow!("File not found :{} ({})", path, e)),
    }
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

fn verify_format_valid(format: &str) -> Result<String, anyhow::Error> {
    match format {
        "json" => Ok(format.to_string()),
        "yaml" => Ok(format.to_string()),
        "toml" => Ok(format.to_string()),
        _ => Err(anyhow::anyhow!("format is not supporteded")),
    }
}

#[derive(Serialize)]
struct TomlRoot {
    #[serde(flatten)]
    data: HashMap<String, Vec<HashMap<String, String>>>,
}

pub fn process_toml(
    record_list: &[HashMap<String, String>],
    toml_key: String,
) -> Result<String, anyhow::Error> {
    let mut root_data = HashMap::new();
    root_data.insert(toml_key.to_string(), record_list.to_vec());
    let tom_root = TomlRoot { data: root_data };

    Ok(toml::to_string_pretty(&tom_root)?)
}

pub fn read_csv_data<R: Read>(
    reader: &mut Reader<R>,
) -> Result<Vec<HashMap<String, String>>, anyhow::Error> {
    let mut record_list: Vec<HashMap<String, String>> = Vec::new();

    let headers = reader.headers()?.clone();

    for (row_index, result) in reader.records().enumerate() {
        let record = result.with_context(|| format!("parse csv row:{} error", row_index + 2))?;

        let mut row = HashMap::new();

        for (col_index, header) in headers.iter().enumerate() {
            if let Some(value) = record.get(col_index) {
                row.insert(header.to_string(), value.to_string());
            } else {
                row.insert(header.to_string(), String::new());
            }
        }

        record_list.push(row);
    }
    Ok(record_list)
}
