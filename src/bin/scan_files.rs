use mame_software_lists::database::establish_connection;
use mame_software_lists::database::roms::set_matched_roms;
use mame_software_lists::files::scan_archives::scan_archives;
use mame_software_lists::software_lists::fetch_software_list::fetch_software_list_roms;
use std::path::PathBuf;

fn handle_args() -> (i32, String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <software list id> <path to files>", args[0]);
        std::process::exit(1);
    }
    let id_str = args[1].clone();
    let id = match id_str.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id: {}", id_str);
            std::process::exit(1);
        }
    };
    let path = args[2].clone();
    (id, path)
}

fn main() {
    let (id, path) = handle_args();
    let connection = &mut establish_connection();

    let roms_in_software_list = match fetch_software_list_roms(connection, id) {
        Ok(software_list) => software_list,
        Err(e) => {
            println!("Error fetching software list: {}", e);
            std::process::exit(1);
        }
    };

    let path = PathBuf::from(path);
    if let Ok(scan_result) = scan_archives(path, roms_in_software_list) {
        let _res = set_matched_roms(connection, &scan_result.matched_rom_ids).map_err(|e| {
            println!("Error setting matched roms: {}", e);
            std::process::exit(1);
        });
    }
}
