use anyhow::{Ok, Result};
use std::{
    fs,
    io::{Read, stdin},
};

pub fn verify_file_exists(path: &str) -> Result<String, anyhow::Error> {
    if path == "-" {
        // 如果输入是 "-", 认为它是有效的特殊值，直接原样返回
        return Ok(path.to_string());
    }
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

pub fn verify_format_valid(format: &str) -> Result<String, anyhow::Error> {
    match format {
        "json" => Ok(format.to_string()),
        "yaml" => Ok(format.to_string()),
        "toml" => Ok(format.to_string()),
        _ => Err(anyhow::anyhow!("format is not supporteded")),
    }
}

pub fn get_reader(key: &str) -> Result<Box<dyn Read>, anyhow::Error> {
    let reader: Box<dyn Read> = if key == "-" {
        Box::new(stdin())
    } else {
        Box::new(fs::File::open(key)?)
    };
    Ok(reader)
}

pub fn get_content(path: &str) -> Result<Vec<u8>, anyhow::Error> {
    let mut reader = get_reader(path)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
