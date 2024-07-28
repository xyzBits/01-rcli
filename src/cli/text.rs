use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::Parser;

use super::verify_file;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    // 如果不写 name，就将小写作为 name
    #[command(name = "sign", about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),

    #[command(name = "verify", about = "Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    // 不加 pub ，外面 使用时无法通过 . 获取
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    // 可以从 std 或者 文件输入，但是只能有一个可以从标准输入获取，key就不要从标准输入读取
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = verify_file, default_value = "-")]
    pub sig: String,
}

#[derive(Clone, Copy, Debug)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> anyhow::Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

// 文本和 enum 之间的转换
// 由 TextSignFormat 转为 &str
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
