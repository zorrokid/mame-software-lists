#[derive(serde::Deserialize, Debug)]
pub struct System {
    pub id: String,
    pub name: String,
    pub extensions: Vec<String>,
}

pub fn read_systems(path: String) -> Vec<System> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}