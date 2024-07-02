use mame_software_lists::systems::systems::read_systems;

fn handle_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to systems.json", args[0]);
        std::process::exit(1);
    }
    args[1].clone()
}

fn main() {

    let path = handle_args();
    let systems = read_systems(path);
    println!("{:?}", systems);
    
}