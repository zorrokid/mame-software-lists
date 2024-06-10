use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rom {
    name: String,
    size: u32,
    crc: String,
    sha1: String,
    #[serde(default)]
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Machine {
    #[serde(rename = "name")]
    name: String,
    description: String,
    year: String,
    publisher: String,
    rom: Vec<Rom>,
}

#[derive(Debug, Deserialize)]
pub struct Header {
    name: String,
    description: String,
    version: String,
    author: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "datafile")]
pub struct DataFile {
    header: Header,
    #[serde(rename = "machine")]
    machines: Vec<Machine>,
}
