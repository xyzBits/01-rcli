use base64::Engine;
use crypto::ed25519;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// 数据结构实现 serde::Serialize trait
/// 可以使用 serde_json 库将 Animal 结构生序列化为JSON 字符串
/// 序列化 struct -> json
/// 反序列化 json -> struct
#[derive(Serialize, Deserialize, Debug)]
struct Animal {
    name: String,
    age: u32,
}

#[test]
fn test_serialize() {
    let animal = Animal {
        name: "tom".to_string(),
        age: 3,
    };

    //{"name":"tom","age":3}
    let json = serde_json::to_string_pretty(&animal).unwrap();
    println!("{}", json);

    let result = serde_json::from_slice::<Animal>(json.as_bytes()).unwrap();
    println!("{:?}", result);
}

#[test]
fn test_slice_choose() {
    // rand::thread_rng 可以获取一个随机数生成器 rand::Rng,
    // 该生成器需要在每个线程都初始化一个
    // 整数的随机分布范围等于类型的取值范围，但是浮点数只分布在 [0, 1) 区间内

    let array = [1, 2, 4, 8, 16, 32];
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let choice = array.choose(&mut rng).unwrap();
        println!("{}", choice);
    }

    let mut rng = rand::thread_rng();
    let mut y = [1, 2, 3, 4, 5];
    println!("Un shuffled: {:?}", y);
    // 将序列的所有元素随机排序
    y.shuffle(&mut rng);
    println!("shuffled: {:?}", y);
}

//Base58
// base58是Base64的一个变种，主要用于数字货币比特币btc中使用的一种特殊的编码方式，主要用于生成钱包地址。主要目的是防止人类误读
// ----不使用数字0和字母大写O，以及字母大写I和字母小写l。
// ----不使用"+"和"/"。
// -----没有标点符号，通常不会被从中间分行。
// -----支持双击选择整个字符串，便于拷贝。
// Base58 的输入是一个[0,256)的值的流，输出结果是一个[0,58) 的值的流。然后将每个值去查字母表，得出一个可视字符串。转换过程实际上就是一个256进制的值转换为一个58进制的值。
//
#[test]
fn test_base58_encode_decode() {
    let data = b"hello world";
    let encode = bs58::encode(data).into_string();

    println!("{}", encode);

    let decode = bs58::decode(encode).into_vec();
    println!("{:?}", String::from_utf8(decode.unwrap()).unwrap());
}

#[test]
fn test_ed25519() {
    // 首先生成一个随机的 32 字节，也就是256位的密钥，
    // 然后使用这个密钥生成一个 Ed25519 的公钥/私钥对
    // 然后将这个公钥/私钥对存储在 wallet 结构中
    let mut key = [0u8; 32];
    let mut rand = OsRng::default();

    // todo  Fill dest with random data.
    rand.fill_bytes(&mut key);
    let (secret_key, public_key) = ed25519::keypair(&key);
    let secret_key = secret_key.to_vec();
    let public_key = public_key.to_vec();
}
