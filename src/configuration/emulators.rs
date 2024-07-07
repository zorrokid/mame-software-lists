use std::error::Error;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Emulator{
    pub id: String,
    pub description: String,
    pub executable: String,
    pub arguments: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct EmulatorsBySystem{
    pub system: String,
    pub emulators: Vec<Emulator>,
}

pub fn get_emulators_by_system_id(system_id: String, emulators_by_system: &Vec<EmulatorsBySystem>) -> Result<Vec<Emulator>, Box<dyn Error>> {
    let emulators_filtered_by_system = emulators_by_system.iter().find(|e| e.system == system_id).unwrap();
    let result = emulators_filtered_by_system.emulators.clone();
    Ok(result)
}

pub fn read_emulators(path: String) -> Vec<EmulatorsBySystem>{
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}