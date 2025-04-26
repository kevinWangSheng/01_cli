use std::path::PathBuf;

#[derive(clap::Subcommand, Debug)]
pub enum Base64Ops {
    /// 将输入数据编码为 Base64 格式
    Encode(IoArgs),
    /// 将 Base64 格式的输入数据解码为原始数据
    Decode(IoArgs),
}

#[derive(clap::Parser, Debug)]
pub struct IoArgs {
    /// 指定输入文件的路径。如果未指定且未使用 --string，则从标准输入读取。
    #[clap(short, long, value_parser = clap::value_parser!(PathBuf))]
    pub input: Option<PathBuf>,

    /// 直接从命令行提供输入字符串。
    #[clap(short = 's', long)] // 使用 -s 作为短标志
    pub string: Option<String>,
}
