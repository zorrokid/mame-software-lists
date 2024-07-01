use mame_software_lists::files::scan_archives::scan_archives;

fn handle_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to files>", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}

fn main() {
    scan_archives(handle_args());
}