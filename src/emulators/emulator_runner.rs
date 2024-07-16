use std::process::Command;

use crate::archives::zip_util::extract_zip_file_to_tmp;
use crate::configuration::emulators::get_emulators_by_system_id;
use crate::configuration::paths::read_paths;
use crate::models::{Machine, Rom};

pub struct EmulatorRunnerError {
    pub message: String,
}

pub fn run_with_emulator(
    machine: &Machine,
    system_id: String,
    emulator_id: String,
    rom: Option<Rom>,
) -> Result<(), EmulatorRunnerError> {
    let emulators_for_system =
        get_emulators_by_system_id(system_id.clone()).map_err(|_| EmulatorRunnerError {
            message: format!("No emulators found for system {}", system_id),
        })?;
    let emulator = emulators_for_system
        .iter()
        .find(|e| e.id == emulator_id)
        .unwrap();

    let paths = read_paths();
    let roms_path = paths.software_lists_roms_folder.clone();
    let file_path = get_machine_file_path(&machine, &system_id, &roms_path)?;
    let mut run_path = file_path.clone();

    if emulator.extract {
        let temp_dir = std::env::temp_dir();
        let file_names =
            extract_zip_file_to_tmp(&file_path, &temp_dir).map_err(|e| EmulatorRunnerError {
                message: format!("Error extracting zip file: {}", e.message),
            })?;
        run_path = if rom.is_some() {
            let rom_name = rom.unwrap().name.clone();
            let normalized_rom_name = normalize_name(&rom_name);
            file_names
                .iter()
                .find(|f| normalize_name(f).ends_with(&normalized_rom_name))
                .unwrap()
                .clone()
        } else {
            file_names.first().unwrap().clone()
        };
    }

    let run_arguments = generate_arguments(emulator.arguments.clone(), run_path.clone());
    Command::new(emulator.executable.clone())
        .args(run_arguments.clone())
        .spawn()
        .map(|_| ())
        .map_err(|e| EmulatorRunnerError {
            message: format!("Error running emulator: {}", e.to_string()),
        })
}

fn normalize_name(name: &String) -> String {
    name.replace("/", "").replace("\\", "")
}

fn get_machine_file_name(machine: &crate::models::Machine) -> String {
    let mut filename = machine.name.clone();
    filename.push_str(".zip");
    filename
}

pub fn get_machine_file_path(
    machine: &crate::models::Machine,
    system_id: &String,
    roms_path: &String,
) -> Result<String, EmulatorRunnerError> {
    let filename = get_machine_file_name(machine);
    let path = format!("{}/{}/{}", roms_path, system_id, filename);

    if std::path::Path::new(&path).exists() {
        return Ok(path.clone());
    }

    Err(EmulatorRunnerError {
        message: format!("Machine file not found in path {}", path),
    })
}

fn generate_arguments(arguments: Vec<String>, file_path: String) -> Vec<String> {
    if arguments.iter().find(|a| a.contains("$PATH")).is_none() {
        let mut arguments = arguments.clone();
        arguments.push(file_path.clone());
        return arguments;
    }
    let mut result = Vec::new();
    for arg in arguments {
        if arg.contains("$PATH") {
            result.push(arg.replace("$PATH", &file_path));
        } else {
            result.push(arg);
        }
    }
    result
}
