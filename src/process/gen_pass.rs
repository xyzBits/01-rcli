use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

// const 类型必须要指定，这里也不用指定 生命周期为 'static ，
const UPPER: &'static [u8] = b"ABCDEFGHGKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghigkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// 完全独立于 cli 的代码
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*NUMBER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);
    // todo: make sure the password has at least one of each type

    let password = String::from_utf8(password)?;
    println!("password = {}", password);

    // output password strength in stderr
    let estimate = zxcvbn(&password, &[])?;
    eprintln!("Password strength: {}", estimate.score());
    Ok(())
}