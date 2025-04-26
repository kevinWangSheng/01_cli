use crate::cli::Opts;
use anyhow::{Context, Result};
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use clap::Parser;
use first_cli::cli::base64::Base64Ops;
use first_cli::cli::csv::OutputFormat; // 导入 OutputFormat
use first_cli::cli::{self, SubCommand}; // 导入 cli 模块和 SubCommand 枚举
use first_cli::process::text::{process_text_sign, process_text_verify};
use first_cli::utils::{get_content, get_reader};
use process::password_generate::password_gen;
use process::text::process_key_generate;
use process::{
    b64::{handle_decode, handle_encode},
    csv_generate::{process_toml, read_csv_data},
};
use std::fs;
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
        SubCommand::Base64(cmd) => {
            match cmd {
                Base64Ops::Encode(args) => {
                    handle_encode(args).context("执行编码时出错")?; // 添加错误上下文
                }
                Base64Ops::Decode(args) => {
                    handle_decode(args).context("执行解码时出错")?; // 添加错误上下文
                }
            }
        }
        SubCommand::Text(text) => {
            match text {
                cli::text::TextSubcommand::Sign(sign) => {
                    let mut reader = get_reader(&sign.input)?;
                    let content = get_content(&sign.key)?;
                    let sign = process_text_sign(&mut reader, &content, sign.format)?;
                    // sign
                    let encoded = BASE64_URL_SAFE_NO_PAD.encode(sign);
                    eprintln!("the signature is :{:?}", encoded);
                }
                cli::text::TextSubcommand::Verify(verify) => {
                    let mut reader = get_reader(&verify.input)?;
                    let content = get_content(&verify.key)?;
                    let decoded = BASE64_URL_SAFE_NO_PAD.decode(&verify.signature)?;
                    let verified =
                        process_text_verify(&mut reader, &content, &decoded, verify.format)?;
                    if verified {
                        println!("✓ Signature verified");
                    } else {
                        println!("⚠ Signature not verified");
                    }
                }
                cli::text::TextSubcommand::GenerateKeyPair(opts) => {
                    let key = process_key_generate(&opts.format)?;
                    for (k, v) in key {
                        fs::write(opts.output_path.join(k), v)?;
                    }
                }
            }
        }
    }

    Ok(())
}
