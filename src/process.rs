use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    // Result 使用 ？ 在内部作  match 处理 Ok(v) Err(e) 其他 error 可以转换为 anyhow的error
    let mut reader = Reader::from_path(input)?; // std::result::Result -> anyhow::Result

    let mut ret = Vec::with_capacity(128);

    // let headers = reader.headers()?;
    // mutable reader.headers() 也是可变引用
    let headers = reader.headers()?.clone();

    // for result in reader.deserialize::<Player>() {
    for result in reader.records() {// reader.records() 也是 可变引用，多个可变引用不能共存
        // let player: Player = result?;

        // 不依赖于具体的数据类型，将 csv 转成 json
        // StringRecord 中并不包含 key，只有对应的 value
        // StringRecord(["Wojciech Szczesny", "Goalkeeper", "Apr 18, 1990 (29)", "Poland", "1"])
        let record = result?;


        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<Value>();

        println!("{:?}", record);
        ret.push(json_value);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?; // => ()
    Ok(())
}
