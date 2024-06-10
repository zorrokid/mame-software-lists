mod file_reader;
mod xml_parser;

use file_reader::read_dir;

fn handle_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to software list>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}



fn main() {
    let path = handle_args();
    read_dir(&path);
}
