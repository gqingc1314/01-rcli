use std::path::Path;

use clap::Parser;

mod base64;
mod csv;
mod genpass;
pub use base64::{Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64SubCommand};
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;

#[derive(Parser, Debug)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommandCmd,
}
#[derive(Debug, Parser)]
pub enum SubCommandCmd {
    #[command(name = "csv", about = "Convert a CSV file to other formats")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
