use crate::cli::password::*;
use anyhow::bail;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng,
};

#[allow(dead_code)]
pub fn password_gen(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> Result<String, anyhow::Error> {
    // generate the password
    if length == 0 {
        return Err(anyhow::anyhow!("Password length cannot be zero"));
    }

    let mut requeire_chars = Vec::new();
    let mut rng = thread_rng(); // <-- 使用 thread_rng() 获取 RNG
    let mut char_pool = String::new();
    if uppercase {
        char_pool.push_str(UPPER_CASE);
        requeire_chars.push(
            UPPER_CASE
                .chars()
                .choose(&mut rng)
                .expect("uppercase won't be empty"),
        );
    }
    if lowercase {
        char_pool.push_str(LOWER_CASE);
        requeire_chars.push(
            LOWER_CASE
                .chars()
                .choose(&mut rng)
                .expect("lowercase won't be empty"),
        );
    }
    if digits {
        char_pool.push_str(DIGITS);
        requeire_chars.push(
            DIGITS
                .chars()
                .choose(&mut rng)
                .expect("digits won't be empty"),
        );
    }
    if symbols {
        char_pool.push_str(SYMBOLS);
        requeire_chars.push(
            SYMBOLS
                .chars()
                .choose(&mut rng)
                .expect("symbols won't be empty"),
        );
    }

    if length < requeire_chars.len() {
        bail!("the generate password length won't less than require length")
    }

    // start generate the password
    let remind_length = length - requeire_chars.len();

    for _ in 0..remind_length {
        requeire_chars.push(
            char_pool
                .chars()
                .choose(&mut rng)
                .expect("char pool won't be empty"),
        );
    }
    requeire_chars.shuffle(&mut rng);
    Ok(requeire_chars.into_iter().collect())
}
