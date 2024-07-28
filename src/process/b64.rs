use std::io::Read;

use anyhow::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;

use crate::{get_reader, Base64Format};

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

#[cfg(test)]
mod tests {
    use base64::prelude::BASE64_STANDARD;
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


    #[test]
    fn test_base64_encode_decode() {
        let data = b"hello world";
        let encode = BASE64_STANDARD.encode(data);
        //aGVsbG8gd29ybGQK
        println!("{}", encode);

        let decode = BASE64_STANDARD.decode(encode.as_bytes()).unwrap();
        println!("{:?}", String::from_utf8(decode).unwrap());
    }
}
