use anyhow::{Context, Result, anyhow}; // 引入 anyhow
use blake3;
use ed25519_dalek::{
    PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH, Signature, Signer, SigningKey,
    Verifier, VerifyingKey,
};
use rand::{Rng, rngs::OsRng};
use std::{collections::HashMap, io::Read};

// 假设 TextFormat 定义在 crate::cli::text 模块下
use crate::cli::text::TextFormat;
// 假设 password_generate 定义在父模块下的 password_generate 子模块

// --- Trait 定义 ---
pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

// --- 结构体定义 ---
pub struct Blake3 {
    key: [u8; 32], // blake3 key size is 32 bytes
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

// --- Blake3 实现 ---
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    // 更安全的 try_new
    pub fn try_new(key_bytes: &[u8]) -> Result<Self> {
        let key: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| anyhow!("Blake3 key must be exactly 32 bytes long"))?;
        Ok(Self::new(key))
    }

    pub fn generate_key() -> Result<HashMap<&'static str, Vec<u8>>> {
        // 使用 CSPRNG 生成更安全的随机密钥，而不是密码生成函数
        let mut key = [0u8; 32];
        rand::thread_rng().fill(&mut key); // 使用线程本地 RNG 填充
        let mut map = HashMap::new();
        map.insert("blake3.key", key.to_vec()); // 直接存储字节
        Ok(map)
    }
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let mac = blake3::keyed_hash(&self.key, &buf);
        Ok(mac.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let expected_mac = blake3::keyed_hash(&self.key, &buf);
        // 使用安全比较，避免时序攻击 (虽然对于 blake3 MAC 可能不是最关键，但好习惯)
        Ok(constant_time_eq::constant_time_eq(
            expected_mac.as_bytes(),
            sig,
        ))
    }
}

// --- Ed25519Signer 实现 ---
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    // 更安全的 try_new
    pub fn try_new(key_bytes: &[u8]) -> Result<Self> {
        // SigningKey::from_bytes 需要一个 32 字节的种子
        let signing_key_bytes: [u8; SECRET_KEY_LENGTH] = key_bytes.try_into().map_err(|_| {
            anyhow!(
                "Ed25519 signing key (seed) must be exactly {} bytes long",
                SECRET_KEY_LENGTH
            )
        })?;
        let signing_key = SigningKey::from_bytes(&signing_key_bytes); // 从种子创建
        Ok(Self::new(signing_key))
    }

    pub fn generate_key() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng; // 使用密码学安全的随机数生成器
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into(); // 或者 sk.verifying_key()
        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec()); // 私钥种子 (32 bytes)
        map.insert("ed25519.pk", pk.to_bytes().to_vec()); // 公钥 (32 bytes)
        Ok(map)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).context("读取输入数据失败")?;
        let signature: Signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec()) // Signature is 64 bytes
    }
}

// --- Ed25519Verifier 实现 ---
impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    // 更安全的 try_new
    pub fn try_new(key_bytes: &[u8]) -> Result<Self> {
        // VerifyingKey::from_bytes 需要一个 32 字节的公钥编码
        let verifying_key_bytes: [u8; PUBLIC_KEY_LENGTH] = key_bytes.try_into().map_err(|_| {
            anyhow!(
                "Ed25519 verifying key must be exactly {} bytes long",
                PUBLIC_KEY_LENGTH
            )
        })?;
        // from_bytes 会检查编码是否有效，返回 Result
        let verifying_key = VerifyingKey::from_bytes(&verifying_key_bytes)
            .map_err(|e| anyhow!("无效的 Ed25519 公钥字节: {}", e))?;
        Ok(Self::new(verifying_key))
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).context("读取输入数据失败")?;

        let signature = Signature::try_from(sig).map_err(|e| {
            anyhow!(
                "无法从字节解析 Ed25519 签名 (长度必须为 {} 字节): {}",
                SIGNATURE_LENGTH, // 使用常量
                e
            )
        })?;

        // verify 返回 Result<(), SignatureError>
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

// --- 处理函数 ---
pub fn process_text_sign(input: &mut dyn Read, key: &[u8], format: TextFormat) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
        // 可以考虑添加其他格式或返回错误
        // _ => bail!("不支持的签名格式: {:?}", format),
    };

    signer.sign(input)
}
pub fn process_text_verify(
    input: &mut dyn Read,
    key: &[u8], // 注意：对于 Ed25519，这里应该是公钥
    sig: &[u8], // 修正参数名 sig
    format: TextFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?), // 使用公钥创建 Verifier
                                                                         // _ => bail!("不支持的验证格式: {:?}", format),
    };

    verifier.verify(input, sig) // 使用修正后的 sig 参数
}

pub fn process_key_generate(format: &TextFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextFormat::Blake3 => Blake3::generate_key(),
        TextFormat::Ed25519 => Ed25519Signer::generate_key(), // 生成密钥对
                                                              // _ => bail!("不支持的密钥生成格式: {:?}", format),
    }
}

// 建议添加 constant_time_eq crate 用于 Blake3 的安全比较
// cargo add constant_time_eq
use constant_time_eq;
// 使用 ed25519_dalek 提供的常量使代码更清晰

#[cfg(test)]
mod tests {
    use super::*; // 导入当前模块的所有项
    use crate::cli::text::TextFormat; // 导入 TextFormat 枚举
    use ed25519_dalek::{SigningKey, VerifyingKey}; // 导入 ed25519 相关
    use rand::rngs::OsRng;
    use std::io::Cursor; // 用于创建内存中的 Reader // 导入 OsRng 用于生成密钥

    // --- 测试 Blake3 ---
    #[test]
    pub fn test_blake3_sign_verify_correct() -> Result<()> {
        let key = [42u8; 32]; // 固定测试密钥
        let message = b"This is a test message for blake3";
        let mut input_reader = Cursor::new(message); // 从字节创建 Reader

        // 签名
        let sig = process_text_sign(&mut input_reader, &key, TextFormat::Blake3)?;

        // 验证 (需要重置 Reader 或创建新的)
        let mut verify_reader = Cursor::new(message);
        let is_valid = process_text_verify(&mut verify_reader, &key, &sig, TextFormat::Blake3)?;

        assert!(is_valid, "Blake3 signature should verify correctly");
        Ok(())
    }

    #[test]
    pub fn test_blake3_verify_incorrect_sig() -> Result<()> {
        let key = [42u8; 32];
        let message = b"Another message";
        let mut input_reader = Cursor::new(message);
        let mut incorrect_sig = process_text_sign(&mut input_reader, &key, TextFormat::Blake3)?;
        // 篡改签名
        incorrect_sig[0] = incorrect_sig[0].wrapping_add(1);

        let mut verify_reader = Cursor::new(message);
        let is_valid =
            process_text_verify(&mut verify_reader, &key, &incorrect_sig, TextFormat::Blake3)?;

        assert!(
            !is_valid,
            "Blake3 signature should not verify with incorrect sig"
        );
        Ok(())
    }

    #[test]
    pub fn test_blake3_verify_incorrect_message() -> Result<()> {
        let key = [42u8; 32];
        let message = b"Original message";
        let tampered_message = b"Tampered message";
        let mut input_reader = Cursor::new(message);

        let sig = process_text_sign(&mut input_reader, &key, TextFormat::Blake3)?;

        let mut verify_reader = Cursor::new(tampered_message); // 使用篡改后的消息
        let is_valid = process_text_verify(&mut verify_reader, &key, &sig, TextFormat::Blake3)?;

        assert!(
            !is_valid,
            "Blake3 signature should not verify with incorrect message"
        );
        Ok(())
    }

    // --- 测试 Ed25519 ---
    #[test]
    pub fn test_ed25519_sign_verify_correct() -> Result<()> {
        // 生成临时的密钥对
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let sk_bytes = sk.to_bytes(); // 32 字节种子
        let pk_bytes = pk.to_bytes(); // 32 字节公钥

        let message = b"Test message for Ed25519 signature";
        let mut input_reader = Cursor::new(message);

        // 签名 (使用私钥种子)
        let sig = process_text_sign(&mut input_reader, &sk_bytes, TextFormat::Ed25519)?;

        // 验证 (使用公钥)
        let mut verify_reader = Cursor::new(message);
        let is_valid =
            process_text_verify(&mut verify_reader, &pk_bytes, &sig, TextFormat::Ed25519)?;

        assert!(is_valid, "Ed25519 signature should verify correctly");
        Ok(())
    }

    #[test]
    pub fn test_ed25519_verify_incorrect_sig() -> Result<()> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let sk_bytes = sk.to_bytes();
        let pk_bytes = pk.to_bytes();
        let message = b"Another Ed25519 message";
        let mut input_reader = Cursor::new(message);

        let mut incorrect_sig =
            process_text_sign(&mut input_reader, &sk_bytes, TextFormat::Ed25519)?;
        incorrect_sig[0] = incorrect_sig[0].wrapping_add(1); // 篡改签名

        let mut verify_reader = Cursor::new(message);
        let is_valid = process_text_verify(
            &mut verify_reader,
            &pk_bytes,
            &incorrect_sig,
            TextFormat::Ed25519,
        )?;

        assert!(
            !is_valid,
            "Ed25519 signature should not verify with incorrect sig"
        );
        Ok(())
    }

    #[test]
    pub fn test_ed25519_verify_incorrect_message() -> Result<()> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();
        let sk_bytes = sk.to_bytes();
        let pk_bytes = pk.to_bytes();
        let message = b"Original Ed25519 message";
        let tampered_message = b"Tampered Ed25519 message";
        let mut input_reader = Cursor::new(message);

        let sig = process_text_sign(&mut input_reader, &sk_bytes, TextFormat::Ed25519)?;

        let mut verify_reader = Cursor::new(tampered_message); // 使用篡改消息
        let is_valid =
            process_text_verify(&mut verify_reader, &pk_bytes, &sig, TextFormat::Ed25519)?;

        assert!(
            !is_valid,
            "Ed25519 signature should not verify with incorrect message"
        );
        Ok(())
    }

    #[test]
    pub fn test_ed25519_verify_wrong_key() -> Result<()> {
        let mut csprng = OsRng;
        let sk1 = SigningKey::generate(&mut csprng);
        let sk1_bytes = sk1.to_bytes();
        // 使用不同的密钥对进行验证
        let sk2 = SigningKey::generate(&mut csprng);
        let pk2: VerifyingKey = (&sk2).into();
        let pk2_bytes = pk2.to_bytes();

        let message = b"Message signed with key 1";
        let mut input_reader = Cursor::new(message);

        // 用 sk1 签名
        let sig = process_text_sign(&mut input_reader, &sk1_bytes, TextFormat::Ed25519)?;

        // 用 pk2 验证
        let mut verify_reader = Cursor::new(message);
        let is_valid =
            process_text_verify(&mut verify_reader, &pk2_bytes, &sig, TextFormat::Ed25519)?;

        assert!(
            !is_valid,
            "Ed25519 signature should not verify with the wrong public key"
        );
        Ok(())
    }

    // --- 测试密钥生成 (简单检查文件数量和类型) ---
    #[test]
    pub fn test_blake3_key_generation() -> Result<()> {
        let keys = process_key_generate(&TextFormat::Blake3)?;
        assert_eq!(keys.len(), 1, "Blake3 should generate 1 key file map entry");
        assert!(
            keys.contains_key("blake3.key"),
            "Blake3 key file name mismatch"
        );
        assert_eq!(
            keys["blake3.key"].len(),
            32,
            "Blake3 key should be 32 bytes"
        );
        Ok(())
    }

    #[test]
    pub fn test_ed25519_key_generation() -> Result<()> {
        let keys = process_key_generate(&TextFormat::Ed25519)?;
        assert_eq!(
            keys.len(),
            2,
            "Ed25519 should generate 2 key file map entries"
        );
        assert!(
            keys.contains_key("ed25519.sk"),
            "Ed25519 sk file name mismatch"
        );
        assert!(
            keys.contains_key("ed25519.pk"),
            "Ed25519 pk file name mismatch"
        );
        assert_eq!(
            keys["ed25519.sk"].len(),
            32,
            "Ed25519 sk should be 32 bytes"
        );
        assert_eq!(
            keys["ed25519.pk"].len(),
            32,
            "Ed25519 pk should be 32 bytes"
        );
        Ok(())
    }
}
