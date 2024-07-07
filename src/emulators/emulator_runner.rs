use std::error::Error;
use std::process::Command;
use diesel::SqliteConnection;
use serde_json::error;

use crate::database::machines::db_get_machine;
use crate::configuration::emulators::{get_emulators_by_system_id, read_emulators};
use crate::configuration::paths::read_paths;


pub fn run_with_emulator(conn: &mut SqliteConnection, system_id: String, emulator_id: String, software_list_machine_id: i32) -> Result<(), Box<dyn Error>> {
    let emulators = read_emulators("configs/emulators.json".to_string());
    let emulators_for_system = get_emulators_by_system_id(system_id.clone(), &emulators)?;
    let emulator = emulators_for_system.iter().find(|e| e.id == emulator_id).unwrap();
    println!("Running emulator: {}", emulator.description);

    let paths = read_paths("configs/paths.json".to_string());
    let roms_path = paths.software_lists_roms_folder.clone();
    println!("Roms path is: {}", roms_path.clone());

    let machine = db_get_machine(conn, software_list_machine_id.clone())?;
    println!("Machine is: {:?}", machine);

    let file_path = get_machine_file_path(&machine, &system_id, &roms_path)?;
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

pub fn get_machine_file_path(machine: &crate::models::Machine, system_id: &String, roms_path: &String) -> Result<String, Box<dyn Error>>{
    let filename = get_machine_file_name(machine);
    let path = format!("{}/{}/{}", roms_path, system_id, filename);

    if std::path::Path::new(&path).exists() {
        return Ok(path.clone());
    }
    let error_msg = format!("Machine file not found in path {}", path);
    return Err(error_msg.into());

}