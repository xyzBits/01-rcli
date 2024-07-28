use std::fs;
use std::io::Read;

use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::{get_reader, TextSignFormat};

trait TextSign {
    // &[u8] impl Read, so we can
    // sign the data from the reader and return the signature
    // 代码体积小，但是性能一般，但 dispatch 比 io 效率高很多
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    // 产生的代码体大，性能好
    // 在 trait 的接口中，对于 owned 的 value，不需要再额外的加 mut
    // 但是在使用时，需要显式的加 mut
    fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Singer {
    key: [u8; 32],
}

struct Ed25519Verifier {
    key: [u8; 32],
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into().unwrap();
            let signer = Blake3 { key };

            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };

    let signed = URL_SAFE_NO_PAD.encode(&signed);

    println!("signed = {}", signed);

    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // todo : improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // let hash = blake3::hash(&buf).as_bytes();
        let hash = blake3::hash(&buf);
        // 重新绑定后，生命周期到函数结束
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}
