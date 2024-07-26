use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

// rcli csv -i input.csv -o output.json --header --d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    // subcommand 一般使用 enum，解析后使用 match 进行匹配处理
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

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
    // short -
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

// &'static 生命周期和进程是一样的
// fn verify_input_file(filename: &str) -> Result<String, String> {
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into()) // into 将 &str 转为 String
    } else {
        Err("File does not exist")
    }
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
