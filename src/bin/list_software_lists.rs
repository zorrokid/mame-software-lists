use mame_software_lists::database::establish_connection;
use mame_software_lists::database::software_lists::db_get_software_lists;

pub fn main() {
    let connection = &mut establish_connection();
    match db_get_software_lists(connection){
 
        Ok(software_lists) => {
            if software_lists.len() == 0 {
                println!("No software lists in database.");
            }
            for software_list in software_lists {
                println!("id: {}, name: {}, version: {}", software_list.id, software_list.name, software_list.version);
            }
        },
        Err(e) => println!("Error inserting software list: {}", e),
    }
}