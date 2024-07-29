use std::fs;

use clap::Parser;
use zxcvbn::zxcvbn;

use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_key_generate,
    process_text_sign, process_text_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
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
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;

            println!("password = {}", password);

            // output password strength in stderr
            let estimate = zxcvbn(&password, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }

        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("encode = {}", encode);
            }

            Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                let decode = String::from_utf8(decode)?;

                println!("decode = {}", decode);
            }
        },

        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let signature = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("signed = {}", signature);
            }

            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("verified = {}", verified);
            }

            TextSubCommand::Generate(opts) => {
                let key_map = process_text_key_generate(opts.format)?;
                for (key, value) in key_map {
                    fs::write(opts.output_path.join(key), value)?;
                }
            }
        },
    }

    Ok(())
}
