use serde::Deserialize;

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
    pub year: String,
    pub publisher: String,
    pub rom: Vec<Rom>,
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

