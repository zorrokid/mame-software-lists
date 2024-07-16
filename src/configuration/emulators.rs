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

pub struct EmulatorError {
    pub message: String,
}

pub fn get_emulators_by_system_id(system_id: String) -> Result<Vec<Emulator>, EmulatorError> {
    let all_emulators = read_emulators(EMULATORS_CONFIG_PATH.to_string());

    all_emulators
        .iter()
        .find(|e| e.system == system_id)
        .ok_or_else(|| EmulatorError {
            message: format!("No emulators found for system {}", system_id),
        })
        .map(|e| e.emulators.clone())
}

pub fn read_emulators(path: String) -> Vec<EmulatorsBySystem> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}
