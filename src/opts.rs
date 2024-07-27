use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

/// https://juejin.cn/post/7242623208825110586?searchId=20240726205358129C4D8536158F998172
///  clap 的使用方式
/// 0. 先创建一个 struct，其中的字段就是命令行的参数名称
/// 1. 给 struct 添加 Parser 的派生
/// 2. 添加 command ，为了控制命令行展示的行为，也可以不添加
/// 3. 给添加添加 arg，为了控制单个参数的信息，也可以不添加
/// 4. 在main函数中解析参数
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
    // 子命令是另一套参数集合 git config, config 就是子命令
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
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

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,

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
