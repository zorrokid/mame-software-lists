use std::error::Error;

#[derive(serde::Deserialize, Debug)]
pub struct Emulator{
    pub id: String,
    pub description: String,
    pub executable: String,
    pub arguments: Vec<String>,
    pub supported_systems: Vec<String>,
}

pub fn run_with_emulator(system_id: String, emulator_id: String, software_list_machine_id: i32) -> Result<(), Box<dyn Error>> {
    let emulators = read_emulators("configs/emulators.json".to_string());
    match emulators.iter().find(|e| e.id == emulator_id) {
        Some(emulator) => {
            println!("Running emulator: {}", emulator.description);
            println!("System id: {}", system_id);
            println!("Software list machine id: {}", software_list_machine_id);
            Ok(())
        },
        None => Err("Emulator not found".into()),
    }
}

pub fn read_emulators(path: String) -> Vec<Emulator>{
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}