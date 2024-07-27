use std::fs::File;
use std::io::Read;

use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("encode = {}", encode);

    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    // avoid accidental newlines
    let buf = buf.trim();


    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // todo Decode date might not be string (but for this example, we assume it is)
    let decode = String::from_utf8(decode)?;
    println!("decode = {}", decode);
    Ok(())
}


fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    // 不同的数据类型，将他们提升到 dyn trait
    // 通过 Box 来消除两种不同的数据类型，我只关心他们都实现了 Read trait 接口
    // 两个不同的数据类型，用同一个 trait 消除他们之间的差异，让他们归为同一种类型
    let reader: Box<dyn Read> = if input == "-" {
        // if 是一个表达式，可以返回一个值，但是不同分支的返回值类型必须一样
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    Ok(reader)
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}