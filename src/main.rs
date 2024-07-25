use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kib: u8,
}

// rcli csv -i input.csv -o output.json --header --d ','
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    // subcommand 一般使用 enum，解析后使用 match 进行匹配处理
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    // short -
    // long --
    // value_parser 对参数预先进行校验
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    // default_value ，调用了 output.json .into 转换成 String
    output: String,

    #[arg(short, long, default_value_t = ',')] // default_value_t 不用进行转换
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

// anyhow 实现了 大多数 standard 的转换
// 其他类型的 Result 都能转换为 anyhow::Result
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    println!("{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(opts) => {
            // Result 使用 ？ 在内部作  match 处理 Ok(v) Err(e) 其他 error 可以转换为 anyhow的error
            let mut reader = Reader::from_path(opts.input)?;// std::result::Result -> anyhow::Result
            for result in reader.deserialize::<Player>() {
                let player: Player = result?;
                println!("{:?}", player);
            }
        }
    }

    Ok(())
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
