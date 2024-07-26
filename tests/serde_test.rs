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

#[cfg(test)]
mod tests {
    use crate::Animal;

    #[test]
    fn test_serialize() {
        let animal = Animal {
            name: "tom".to_string(),
            age: 3
        };

        //{"name":"tom","age":3}
        let json = serde_json::to_string_pretty(&animal).unwrap();
        println!("{}", json);

        let result = serde_json::from_slice::<Animal>(json.as_bytes()).unwrap();
        println!("{:?}", result);
    }
}

