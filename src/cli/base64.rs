use std::{fmt, str::FromStr};

use super::verify_input_file;
use clap::Parser;
#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=verify_input_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser=verify_input_file,default_value="-")]
    pub input: String,
    #[arg(long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid base64 format: {}", s)),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match self {
        //     Base64Format::Standard => write!(f, "standard"),
        //     Base64Format::UrlSafe => write!(f, "urlsafe"),
        // }
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
