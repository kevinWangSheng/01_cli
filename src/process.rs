use clap::{ArgAction::SetFalse, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
// parse rcli csv --input.csv --output.json() --delimiter=, --header=true --verbose=true
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
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    // this value were use like "input.csv".into() to convert it into a String
    #[arg(short,long,default_value="input.csv",value_parser=verify_file_exists)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,
    #[arg(long,action=SetFalse,default_value_t=true)]
    pub header: bool,
    #[arg(short, long, default_value_t = true)]
    pub verbose: bool,
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
