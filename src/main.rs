use clap::Parser;

use rcli::{process_csv, Opts, SubCommand};

// anyhow 实现了 大多数 standard 的转换
// 其他类型的 Result 都能转换为 anyhow::Result
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    println!("{:?}", opts);

    match opts.cmd {
        // 使用到 opts 中的数据结构，必须是 pub 的
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }

    Ok(())
}
