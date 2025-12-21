use csv::Reader;
// use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

use crate::OutputFormat;
// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Player {
//     // #[serde(rename="Name")]
//     pub name: String,
//     // #[serde(rename="Position")]
//     pub position: String,
//     #[serde(rename = "DOB")]
//     pub dob: String,
//     // #[serde(rename="Nationality")]
//     pub nationality: String,
//     #[serde(rename = "Kit Number")]
//     pub kit: u8,
// }

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        // println!("{:?}",record);
        ret.push(json_value);
    }
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            let mut map = HashMap::new();
            map.insert("items", &ret);
            toml::to_string(&map)?
        }
    };
    // let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, content)?;
    Ok(())
}
// pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
//     let mut reader = Reader::from_path(input)?;
//     let mut ret = Vec::with_capacity(128);
//     for result in reader.deserialize() {
//         let record: Player = result?;
//         ret.push(record);
//         // println!("{:?}",record);
//     }
//     let json = serde_json::to_string_pretty(&ret)?;
//     fs::write(output, json)?;
//     Ok(())
// }
