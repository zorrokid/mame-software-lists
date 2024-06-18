use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Rom {
    pub name: String,
    pub size: u32,
    pub crc: String,
    pub sha1: String,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Machine {
    #[serde(rename = "name")]
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "deserialize_optional_number")]
    pub year: Option<u32>,
    pub publisher: String,
    pub rom: Vec<Rom>,
}

fn deserialize_optional_number<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match u32::from_str(&s) {
        Ok(num) => Ok(Some(num)),
        Err(_) => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "datafile")]
pub struct DataFile {
    pub header: Header,
    #[serde(rename = "machine")]
    pub machines: Vec<Machine>,
}

