use clap::Parser;
use std::path::{Path, PathBuf};

pub use base64::*;
pub use csv::*;
pub use genpass::*;
pub use http::*;
pub use text::*;

mod base64;
mod csv;
mod genpass;
mod http;
mod text;

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

    // #[command(name = "base64", about = "Encode or decode base64")]
    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}

// &'static 生命周期和进程是一样的
// fn verify_file(filename: &str) -> Result<String, String> {
fn verify_file(filename: &str) -> anyhow::Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into()) // into 将 &str 转为 String
    } else {
        Err("File does not exist")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

// cfg 不会将test编译进去
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File does not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File does not exist"));
    }
}
