use mame_software_lists::emulators::emulator_runner::run_with_emulator;
fn handle_args() -> (String, String, i32){
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <system id> <emulator id> <software list machine id>", args[0]);
        std::process::exit(1);
    }
    let system_id = args[1].clone();
    let emulator_id = args[2].clone();
    let software_list_machine_id_str = args[3].clone();
    let software_list_machine_id = match software_list_machine_id_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id: {}", software_list_machine_id_str);
            std::process::exit(1);
        }
    };
    return (system_id, emulator_id, software_list_machine_id);
}

fn main() {
    let (system_id, emulator_id, software_list_machine_id) = handle_args();
    match run_with_emulator(system_id, emulator_id, software_list_machine_id) {
        Ok(_) => println!("Emulator ran successfully"),
        Err(e) => println!("Error running emulator: {}", e),
    }
}