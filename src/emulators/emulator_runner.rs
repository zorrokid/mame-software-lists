use std::error::Error;
use diesel::SqliteConnection;

use crate::database::machines::db_get_machine;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Emulator{
    pub id: String,
    pub description: String,
    pub executable: String,
    pub arguments: Vec<String>,
    pub supported_systems: Vec<String>,
}

pub fn run_with_emulator(conn: &mut SqliteConnection, system_id: String, emulator_id: String, software_list_machine_id: i32) -> Result<(), Box<dyn Error>> {
    let emulator = get_emulator_by_id(emulator_id.clone())?;
    println!("Running emulator: {}", emulator.description);
    if !emulator.supported_systems.contains(&system_id) {
        return Err("System not supported by emulator".into());
    }
    let machine = db_get_machine(conn, software_list_machine_id.clone())?;

    println!("Software list machine: {}", machine.name);
    Ok(())
}

pub fn get_emulator_by_id(emulator_id: String) -> Result<Emulator, Box<dyn Error>> {
    let emulators = read_emulators("configs/emulators.json".to_string());
    match emulators.iter().find(|e| e.id == emulator_id) {
        Some(emulator) => Ok(emulator.clone()),
        None => Err("Emulator not found".into()),
    }
}

pub fn read_emulators(path: String) -> Vec<Emulator>{
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}