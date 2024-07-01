use mame_software_lists::database::establish_connection;
use mame_software_lists::software_lists::fetch_software_list::fetch_software_list_roms;

fn handle_args() -> i32{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <id of software list>", args[0]);
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
    return id;
}

fn main() {
    let id = handle_args();
    let connection = &mut establish_connection();
    match fetch_software_list_roms(connection, id){
        Ok(roms_in_software_list) => {
            println!("{:?}", roms_in_software_list);
        },
        Err(e) => println!("Error fetching software list: {}", e),
    }
}