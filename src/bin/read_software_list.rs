use mame_software_lists::database::insert_software_list;
use mame_software_lists::database::establish_connection;
use mame_software_lists::xml_parser::parse_file;
use mame_software_lists::models;

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
    let path = handle_args();
    match parse_file(&path){
        Ok(datafile) => {
            println!("{:?}", datafile);
            let software_list: models::SoftwareList = datafile.header.into();
            match insert_software_list(connection, software_list){
                Ok(_) => println!("Inserted software list"),
                Err(e) => println!("Error inserting software list: {}", e),
            }
        },
        Err(e) => println!("Error parsing file: {}", e),
    }

}
