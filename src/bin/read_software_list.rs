use mame_software_lists::data_access::data_access_provider;
use mame_software_lists::software_lists::process::process_from_datafile;

fn handle_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to software list>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}

fn main() {
    let mut data_access_provider = data_access_provider::DataAccessProvider::new();
    match process_from_datafile(&mut data_access_provider, handle_args()) {
        Ok(_) => println!("Software list processed"),
        Err(e) => println!("Error processing software list: {}", e.message),
    }
}
