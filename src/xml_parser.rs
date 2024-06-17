use std::fs::File;
use std::io::BufReader;
use serde_xml_rs::from_reader;

use super::software_list_models::DataFile;

pub fn parse_file(path: &str) -> Result<DataFile, serde_xml_rs::Error> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let datafile: DataFile = from_reader(file)?;
    Ok(datafile)
}