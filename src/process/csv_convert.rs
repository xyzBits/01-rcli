use std::fmt::{Display, Formatter};
use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 不用在每个域上进行处理，只需要对特定的域进行处理，例如 DOB Kit Number
pub struct Player {
    // #[serde(rename = "Name")]
    name: String,

    // #[serde(rename = "Position")]
    position: String,

    #[serde(rename = "DOB")]
    dob: String,

    // #[serde(rename = "Nationality")]
    nationality: String,

    #[serde(rename = "Kit Number")]
    kib: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    // Result 使用 ？ 在内部作  match 处理 Ok(v) Err(e) 其他 error 可以转换为 anyhow的error
    let mut reader = Reader::from_path(input)?; // std::result::Result -> anyhow::Result

    let mut ret = Vec::with_capacity(128);

    // let headers = reader.headers()?;
    // mutable reader.headers() 也是可变引用
    let headers = reader.headers()?.clone();
    // headers = StringRecord(["Name", "Position", "DOB", "Nationality", "Kit Number"])
    println!("headers = {:?}", headers);

    // for result in reader.deserialize::<Player>() {
    for result in reader.records() {
        // reader.records() 也是 可变引用，多个可变引用不能共存
        // let player: Player = result?;

        // 不依赖于具体的数据类型，将 csv 转成 json
        // StringRecord 中并不包含 key，只有对应的 value
        // StringRecord(["Wojciech Szczesny", "Goalkeeper", "Apr 18, 1990 (29)", "Poland", "1"])
        let record = result?;

        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() -> 将两个迭代器合并为一个元组的迭代器，[(header, record), ...]
        // collect::<Value>() -> 将元组转换为 JSON value
        // 这样处理后，json 解析不会和结构体 Player 绑定
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();

        println!("{:?}", record);
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
    };
    // let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, content)?; // => ()
    Ok(())
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

#[cfg(test)]
mod tests {
    use csv::StringRecord;
    use serde_json::Value;

    #[test]
    fn test_iter_zip() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];

        let c = a.into_iter().zip(b.into_iter()).collect::<Vec<_>>();
        println!("{:?}", c);

        // headers = StringRecord(["Name", "Position", "DOB", "Nationality", "Kit Number"])

        let mut headers =
            StringRecord::from(vec!["Name", "Position", "DOB", "Nationality", "Kit Number"]);
        let mut record = StringRecord::from(vec![
            "Wojciech Szczesny",
            "Goalkeeper",
            "Apr 18, 1990 (29)",
            "Poland",
            "1",
        ]);

        let json_value = headers.iter().zip(record.iter()).collect::<Value>();

        println!("{}", serde_json::to_string_pretty(&json_value).unwrap());
    }
}
