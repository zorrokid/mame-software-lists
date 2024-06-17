use mame_software_lists::database::establish_connection;
use mame_software_lists::database::software_lists::delete_software_list;

fn handle_args() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <name> <version>", args[0]);
        std::process::exit(1);
    }
    (args[1].clone(), args[2].clone())
}


fn main(){
    let args = handle_args();
    let connection = &mut establish_connection();
    match delete_software_list(connection, args.0, args.1) {
        Ok(_) => println!("Deleted software list"),
        Err(e) => println!("Error deleting software list: {}", e),
    }
}