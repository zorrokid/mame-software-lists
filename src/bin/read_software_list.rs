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
    process_from_datafile(handle_args())
}
