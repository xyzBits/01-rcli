use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

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

    for result in reader.deserialize::<Player>() {
        let player: Player = result?;
        ret.push(player);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?; // => ()
    Ok(())
}
