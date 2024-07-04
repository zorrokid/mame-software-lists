use std::error::Error;
use std::process::Command;
use diesel::SqliteConnection;

use crate::database::machines::db_get_machine;
use crate::systems::systems::get_system_by_id;

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
    let system = get_system_by_id(system_id.clone())?;
    let machine = db_get_machine(conn, software_list_machine_id.clone())?;
    let file_path = get_machine_file_path(&machine, &system)?;
    println!("Running machine: {}", machine.name);
    println!("File path: {}", file_path.clone());
    println!("Running emulator {} with arguments {:?}", emulator.executable, emulator.arguments);
    let output = Command::new(emulator.executable.clone())
        .args(emulator.arguments.clone())
        .arg(file_path.clone())
        .output()?;

    if output.status.success() == false {
        let error_message = String::from_utf8(output.stderr).unwrap();
        return Err(error_message.into());
    } 

    return Ok(());
}

fn get_machine_file_name(machine: &crate::models::Machine) -> String {
    let mut filename = machine.name.clone();
    filename.push_str(".zip");
    filename
}

pub fn get_machine_file_path(machine: &crate::models::Machine, system: &crate::systems::systems::System) -> Result<String, Box<dyn Error>>{
    let filename = get_machine_file_name(machine);
    for path in system.file_paths.iter() {
        let full_path = format!("{}/{}", path, filename);
        if std::path::Path::new(&full_path).exists() {
            return Ok(full_path);
        }
    }
    return Err("Machine file not found".into());
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