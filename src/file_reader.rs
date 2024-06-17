/*use super::xml_parser::parse_file;
use super::database::insert_datafile;

pub fn read_dir(path: &str, conn: &Connection) {
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{}", path.display());
                        if path.extension().map_or(false, |ext| ext == "xml") {
                            match path.to_str() {
                                Some(path) => {
                                   let result = parse_file(path);
                                   match result {
                                       Ok(_) => insert_datafile(&conn, &result.unwrap()),
                                       Err(e) => println!("Error parsing file: {}", e),
                                   }
                                   
                                   println!("{:?}", result);
                                }
                                None => println!("Error converting path to string"),
                            }
                        }
                    }
                    Err(e) => println!("Error reading entry: {}", e),
                }
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}
*/