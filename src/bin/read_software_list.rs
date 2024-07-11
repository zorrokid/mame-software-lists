use mame_software_lists::software_lists::process::process_from_datafile;
use mame_software_lists::database::establish_connection;

fn handle_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to software list>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}

fn main() {
    let connection = &mut establish_connection();
    process_from_datafile(connection, handle_args())
}
