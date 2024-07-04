#[derive(serde::Deserialize, Debug, Clone)]
pub struct System {
    pub id: String,
    pub name: String,
    pub extensions: Vec<String>,
    pub file_paths: Vec<String>,
}

pub fn read_systems(path: String) -> Vec<System> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn get_system_by_id(system_id: String) -> Result<System, Box<dyn std::error::Error>> {
    let systems = read_systems("configs/systems.json".to_string());
    match systems.iter().find(|s| s.id == system_id) {
        Some(system) => Ok(system.clone()),
        None => Err("System not found".into()),
    }
}