use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommandCmd,
};

//rcli csv -i input.csv -o output.json -header -d ','

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommandCmd::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // "output.json".into()
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        SubCommandCmd::GenPass(opts) => process_genpass(&opts)?,
        SubCommandCmd::Base64(Base64SubCommand::Encode(opts)) => {
            process_encode(&opts.input, opts.format)?
        }
        SubCommandCmd::Base64(Base64SubCommand::Decode(opts)) => {
            process_decode(&opts.input, opts.format)?
        }
    }
    Ok(())
}
