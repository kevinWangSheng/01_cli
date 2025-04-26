use super::{super::utils::verify_file_exists, verify_path};
use clap::Parser;
use std::{path::PathBuf, str::FromStr};
#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(SignText),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(VerifyText),
    #[command(about = "Generate a new key pair")]
    GenerateKeyPair(GenerateKeyPair),
}

#[derive(Debug, Parser)]
pub struct SignText {
    #[arg(long,value_parser=verify_file_exists,default_value = "-")]
    pub input: String,
    #[arg(long,value_parser=verify_file_exists,default_value = "-")]
    pub key: String,
    #[arg(long,value_parser=parser_format,default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyText {
    #[arg(long,value_parser=verify_file_exists,default_value = "-")]
    pub input: String,
    #[arg(long,value_parser=verify_file_exists,default_value = "-")]
    pub key: String,

    pub signature: String,
    #[arg(long,value_parser=parser_format,default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateKeyPair {
    #[arg(long,value_parser=verify_path,default_value = "-")]
    pub output_path: PathBuf,
    #[arg(long,value_parser=parser_format,default_value = "blake3")]
    pub format: TextFormat,
}

#[derive(Debug, Clone)]
pub enum TextFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextFormat::Blake3),
            "ed25519" => Ok(TextFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid signature format")),
        }
    }
}

fn parser_format(format: &str) -> Result<TextFormat, anyhow::Error> {
    format.parse()
}
