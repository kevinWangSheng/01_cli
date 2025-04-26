use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Write},
};

use anyhow::{Context, Result, bail};
use base64::{Engine, prelude::BASE64_STANDARD, read::DecoderReader, write::EncoderWriter};

use crate::cli::base64::IoArgs;

// --- 编码处理函数 ---
pub fn handle_encode(args: IoArgs) -> Result<()> {
    // 获取标准输出的句柄，并用 BufWriter 包装以提高效率
    let mut writer = BufWriter::new(io::stdout());

    // --- 根据参数选择输入源并执行编码 ---
    match (&args.input, &args.string) {
        // 错误情况：同时指定了文件和字符串输入
        (Some(_), Some(_)) => {
            bail!("错误：不能同时通过 --input 文件和 --string 字符串提供输入。");
        }
        // 情况1：从文件输入
        (Some(path), None) => {
            eprintln!(
                "信息：从文件 '{}' 读取并进行 Base64 编码...",
                path.display()
            );
            let file = File::open(path)
                .with_context(|| format!("无法打开输入文件：'{}'", path.display()))?;
            // 使用 BufReader 提高文件读取效率
            let mut reader = BufReader::new(file);

            // --- 使用流式编码 ---
            // 创建一个 EncoderWriter，它会把写入它的数据进行 Base64 编码，
            // 然后将编码后的文本写入到底层的 writer (这里是 stdout)
            let mut encoder = EncoderWriter::new(&mut writer, &BASE64_STANDARD);
            // 使用 io::copy 高效地从 reader 读取数据块，写入 encoder 进行编码和输出
            let bytes_copied =
                io::copy(&mut reader, &mut encoder).context("在流式编码过程中复制数据失败")?;
            // 必须调用 finish() 来确保所有缓冲的数据被处理，并且写入 Base64 padding ('=')
            encoder
                .finish()
                .context("完成 Base64 编码（写入填充符）失败")?;
            eprintln!("成功编码 {} 字节。", bytes_copied); // 在 stderr 输出状态信息
        }
        // 情况2：从命令行字符串输入
        (None, Some(s)) => {
            eprintln!("信息：对提供的字符串进行 Base64 编码...");
            // --- 使用直接编码 ---
            // 对于字符串这种已知大小且通常不大的输入，可以直接调用 Engine 的 encode 方法
            let encoded_string = BASE64_STANDARD.encode(s.as_bytes());
            // 将编码后的 Base64 字符串直接写入输出
            writer
                .write_all(encoded_string.as_bytes())
                .context("将编码后的字符串写入输出失败")?;
            eprintln!("成功编码 {} 字节的原始数据。", s.len());
        }
        // 情况3：从标准输入 (stdin) 读取
        (None, None) => {
            eprintln!("信息：从标准输入读取并进行 Base64 编码... (按 Ctrl+D 或 Ctrl+Z 结束输入)");
            // 获取标准输入的句柄并锁定
            // 使用 BufReader 包装 stdin
            let mut reader = BufReader::new(io::stdin());

            // --- 使用流式编码 (同文件处理) ---
            let mut encoder = EncoderWriter::new(&mut writer, &BASE64_STANDARD);
            let bytes_copied =
                io::copy(&mut reader, &mut encoder).context("在流式编码过程中复制数据失败")?;
            encoder
                .finish()
                .context("完成 Base64 编码（写入填充符）失败")?;
            eprintln!("成功编码 {} 字节。", bytes_copied);
        }
    }

    // 确保所有缓冲在 BufWriter 中的数据都被写入底层 stdout
    writer.flush().context("刷新标准输出缓冲区失败")?;

    Ok(())
}

// --- 解码处理函数 ---
pub fn handle_decode(args: IoArgs) -> Result<()> {
    // 获取标准输出的句柄，并用 BufWriter 包装
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut writer = BufWriter::new(stdout_lock);

    // --- 根据参数选择输入源并执行解码 ---
    match (&args.input, &args.string) {
        // 错误情况：同时指定了文件和字符串输入
        (Some(_), Some(_)) => {
            bail!("错误：不能同时通过 --input 文件和 --string 字符串提供输入。");
        }
        // 情况1：从文件输入
        (Some(path), None) => {
            eprintln!(
                "信息：从文件 '{}' 读取并进行 Base64 解码...",
                path.display()
            );
            let file = File::open(path)
                .with_context(|| format!("无法打开输入文件：'{}'", path.display()))?;
            let reader = BufReader::new(file); // 输入已经是 Base64 文本了

            // --- 使用流式解码 ---
            // 创建一个 DecoderReader，它会从底层的 reader 读取 Base64 文本，
            // 进行解码，然后提供解码后的原始二进制数据供读取。
            let mut decoder = DecoderReader::new(reader, &BASE64_STANDARD);
            // 使用 io::copy 高效地从 decoder 读取解码后的数据块，写入 writer (stdout)
            // 如果输入包含无效 Base64 字符，DecoderReader 在被读取时会返回错误
            let bytes_copied = io::copy(&mut decoder, &mut writer)
                .context("在流式解码过程中复制数据失败 (请检查输入是否为有效的 Base64)")?;
            eprintln!("成功解码 {} 字节。", bytes_copied);
        }
        // 情况2：从命令行字符串输入
        (None, Some(s)) => {
            eprintln!("信息：对提供的字符串进行 Base64 解码...");
            // --- 使用直接解码 ---
            // 对于字符串输入，可以直接调用 Engine 的 decode 方法
            // decode 返回 Result<Vec<u8>, DecodeError>
            let decoded_bytes = BASE64_STANDARD
                .decode(s.as_bytes())
                .context("解码 Base64 字符串失败 (请检查输入是否为有效的 Base64)")?;
            // 将解码得到的原始字节写入输出
            writer
                .write_all(&decoded_bytes)
                .context("将解码后的字节写入输出失败")?;
            eprintln!("成功解码得到 {} 字节。", decoded_bytes.len());
        }
        // 情况3：从标准输入 (stdin) 读取
        (None, None) => {
            eprintln!("信息：从标准输入读取并进行 Base64 解码... (按 Ctrl+D 或 Ctrl+Z 结束输入)");
            let stdin = io::stdin();
            let stdin_lock = stdin.lock();
            let reader = BufReader::new(stdin_lock); // 输入是 Base64 文本

            // --- 使用流式解码 (同文件处理) ---
            let mut decoder = DecoderReader::new(reader, &BASE64_STANDARD);
            let bytes_copied = io::copy(&mut decoder, &mut writer)
                .context("在流式解码过程中复制数据失败 (请检查输入是否为有效的 Base64)")?;
            eprintln!("成功解码 {} 字节。", bytes_copied);
        }
    }

    // 确保所有缓冲在 BufWriter 中的数据都被写入底层 stdout
    writer.flush().context("刷新标准输出缓冲区失败")?;

    Ok(())
}
