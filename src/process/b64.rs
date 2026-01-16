use crate::Base64Format;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::{fs::File, io::Read};
pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
        Base64Format::Standard => STANDARD.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim_end();
    let decoded = match format {
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
        Base64Format::Standard => STANDARD.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::UrlSafe;
        assert!(process_encode(input, format).is_ok());
    }
    #[test]
    fn test_process_decode() {
        let input = "b64.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
