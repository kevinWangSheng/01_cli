use clap::Parser;
#[derive(Debug, Parser)]
pub struct PasswordOpts {
    #[clap(long, default_value_t = 16)]
    pub length: usize,
    #[clap(long, default_value_t = true)]
    pub uppercase: bool,
    #[clap(long, default_value_t = true)]
    pub lowercase: bool,
    #[clap(long, default_value_t = true)]
    pub digits: bool,
    #[clap(long, default_value_t = true)]
    pub symbols: bool,
}

pub const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const DIGITS: &str = "0123456789";
pub const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
