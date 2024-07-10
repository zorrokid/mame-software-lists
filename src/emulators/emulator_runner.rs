use std::error::Error;
use std::process::Command;

use crate::configuration::emulators::get_emulators_by_system_id;
use crate::configuration::paths::{read_paths, PATHS_CONFIG_PATH};
use crate::models::Machine;


pub fn run_with_emulator(machine: &Machine, system_id: String, emulator_id: String) -> Result<(), Box<dyn Error>> {
    let emulators_for_system = get_emulators_by_system_id(system_id.clone())?;
    let emulator = emulators_for_system.iter().find(|e| e.id == emulator_id).unwrap();
    println!("Running emulator: {}", emulator.description);

    let paths = read_paths(PATHS_CONFIG_PATH.to_string());
    let roms_path = paths.software_lists_roms_folder.clone();
    println!("Roms path is: {}", roms_path.clone());

    println!("Machine is: {:?}", machine);

    let file_path = get_machine_file_path(&machine, &system_id, &roms_path)?;
    println!("File path: {}", file_path.clone());
    println!("Running emulator {} with arguments {:?}", emulator.executable, emulator.arguments);
    let output = Command::new(emulator.executable.clone())
        .args(emulator.arguments.clone())
        .arg(file_path.clone())
        .spawn();


    match output {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Error running emulator: {}", e);
            return Err(e.into());
        }
    }

    
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