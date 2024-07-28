use std::fs::File;
use std::io::Read;

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
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
