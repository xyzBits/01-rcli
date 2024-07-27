use rand::seq::SliceRandom;
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
