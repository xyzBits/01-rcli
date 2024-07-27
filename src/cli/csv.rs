use super::verify_input_file;
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;

// 单一的值，不存在堆上的引用，占用小
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    // toml 暂时不支持
    // Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    // short - 设置单字母选项和长选项
    // long --
    // value_parser 对参数预先进行校验
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    // #[arg(short, long, default_value = "output.json")]
    // // default_value ，调用了 output.json .into 转换成 String
    // pub output: String,
    #[arg(short, long)]
    pub output: Option<String>,

    // 不要 short ，因为 -f 可能会有歧义
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')] // default_value_t 不用进行转换
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::Json),
    //     "yaml" => Ok(OutputFormat::Yaml),
    //     "toml" => Ok(OutputFormat::Toml),
    //     _ => Err("Invalid format")
    // }

    // parse 将 str 解析成其他的数据类型，前提是这个数据类型实现了 fromStr
    format.parse::<OutputFormat>()
}

/// 将 OutputFormat 转为 &str
/// 实现这个 trait 后，也可以将 &str 转为 OutputFormat
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
