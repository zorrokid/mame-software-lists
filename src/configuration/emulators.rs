use std::error::Error;

pub const EMULATORS_CONFIG_PATH: &str = "configs/emulators.json";

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Emulator {
    pub id: String,
    pub description: String,
    pub executable: String,
    pub arguments: Vec<String>,
    pub extract: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct EmulatorsBySystem {
    pub system: String,
    pub emulators: Vec<Emulator>,
}

pub fn get_emulators_by_system_id(system_id: String) -> Result<Vec<Emulator>, Box<dyn Error>> {
    let all_emulators = read_emulators(EMULATORS_CONFIG_PATH.to_string());
    let emulators_filtered_by_system = all_emulators.iter().find(|e| e.system == system_id);
    if emulators_filtered_by_system.is_none() {
        return Err("No emulators found for system".into());
    }
    let result = emulators_filtered_by_system.unwrap().emulators.clone();
    Ok(result)
}

pub fn read_emulators(path: String) -> Vec<EmulatorsBySystem> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}
