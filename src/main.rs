use clap::Parser;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

// anyhow 实现了 大多数 standard 的转换
// 其他类型的 Result 都能转换为 anyhow::Result
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    println!("{:?}", opts);

    match opts.cmd {
        // 使用到 opts 中的数据结构，必须是 pub 的
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // {} 中能够使用，format 需要 impl Display
                format!("output.{}", opts.format)
                // "output.json".into()
            };
            process_csv(&opts.input, output, opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }

            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    Ok(())
}
