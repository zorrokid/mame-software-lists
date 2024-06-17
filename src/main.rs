/*mod file_reader;
mod xml_parser;
mod models;
mod database;

use file_reader::read_dir;
use database::get_headers;

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
    let connection = database::get_connection().expect("Error connecting to database");
    read_dir(&path);
    let headers = get_headers(&connection).expect("Error getting headers");
    println!("Headers:");
    for header in headers {
        println!("{:?}", header);
    }
}
*/
fn main() {
}
