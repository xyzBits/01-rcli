use std::fmt::{Display, Formatter};
use std::str::FromStr;

use clap::Parser;

use super::verify_file;

/// Base64 58 是两种常见的编码方式，
/// 用于将二进制数据转换为可打印字符，
/// base64编码使用26个大写字母，26个小写字母，10个数字和两个特殊字符，+/
///
/// base58是64的一个变种，去掉了容易丢掉与混淆的字符 O 1 I O
/// 只使用剩余的58个字符
/// Base64是一种用64个可打印字符来表示任意二进制数据的方法。
/// 用记事本打开exe、jpg、pdf这些文件时，我们都会看到一大堆乱码，
/// 因为二进制文件包含很多无法显示和打印的字符，所以，如果要让记事本
/// 这样的文本处理软件能处理二进制数据，就需要一个二进制到字符串的
/// 转换方法。Base64是一种最常见的二进制编码方法。

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    // 不加 pub ，外面 使用时无法通过 . 获取
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> anyhow::Result<Base64Format, anyhow::Error> {
    format.parse()
}

// 由 Base64Format 转为 &str
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlSafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlSafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
