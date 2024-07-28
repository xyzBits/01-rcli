use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{get_reader, TextSignFormat};


pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;

            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let singer = Ed25519Singer::load(key)?;
            singer.sign(&mut reader)?
        }
    };

    let signed = URL_SAFE_NO_PAD.encode(&signed);

    println!("signed = {}", signed);

    Ok(())
}


pub fn process_text_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<()> {
    let mut reader = get_reader(input)?;

    let sig = URL_SAFE_NO_PAD.decode(sig)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;

            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };


    println!("verified = {}", verified);

    Ok(())
}



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
    // Verify the data from the reader with the signature
    fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>;
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
    // 返回定长的数据结构
        Self: Sized; // marker trait ，需要有这种行为，说明 Self 是有固定长度的数据结构，str [u8] 这些不是有固定长度的
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Singer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
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

impl TextSign for Ed25519Singer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);

        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Singer {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Singer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        // 生成 SigningKey 的方式 key 和 signer 对应
        let key = SigningKey::from_bytes(key.try_into()?);
        let singer = Ed25519Singer::new(key);
        Ok(singer)
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let singer = Ed25519Verifier::new(key);
        Ok(singer)
    }

    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}


#[cfg(test)]
mod tests {
    use base64::alphabet::STANDARD;
    use rand::rngs::OsRng;
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..]).unwrap();

        println!("{}", URL_SAFE_NO_PAD.encode(&sig));

        assert!(blake3.verify(&data[..], &sig).unwrap());

        Ok(())
    }

    #[test]
    fn test_blake3() {
        let key = b"1LNQ3ny#&Y@q^@8_5VDVzi9w4a_B!@6#";
        let message = b"hello world";
        let sig = blake3::keyed_hash(key, message).as_bytes().to_vec();
        let sig1 = blake3::keyed_hash(key, message).as_bytes().to_vec();
        assert_eq!(sig, sig1);
    }

    #[test]
    fn test_ed25519_sign_and_verify() -> Result<()> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        // let signing_key = SigningKey::from_bytes("hello world".as_bytes().try_into()?);

        let message = b"hello world";
        let signature = signing_key.sign(message);

        assert!(signing_key.verify(message, &signature).is_ok());

        Ok(())
    }

    #[test]
    fn test_ed25519() -> Result<()> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let key = signing_key.as_bytes().to_vec();
        let signer = Ed25519Singer::try_new(&key)?;
        let message = b"hello world";
        let mut signature = signer.sign(&mut &message[..]).unwrap();
        let result = URL_SAFE_NO_PAD.encode(&mut signature);
        println!("{:?}", result);


        let verifier = Ed25519Verifier::try_new(&key)?;

        let result = verifier.verify(&mut &message[..], &signature).is_ok();
        println!("verify result = {}", result);
        Ok(())
    }

}