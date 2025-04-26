use anyhow::{Context, Ok};
use csv::Reader;
use std::{collections::HashMap, io::Read};

use crate::cli::csv::TomlRoot;

// parse rcli csv --input.csv --output.json() --delimiter=, --header=true --verbose=true

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
