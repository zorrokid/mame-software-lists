use mame_software_lists::database::establish_connection;
use mame_software_lists::database::software_lists::db_get_software_lists;
use mame_software_lists::software_lists::fetch_software_list::fetch_software_list;

fn main() {
    let connection = &mut establish_connection();
    match db_get_software_lists(connection){
        Ok(software_lists) => {
            if software_lists.len() == 0 {
                println!("No software lists in database.");
            }
            for software_list in software_lists {
                match fetch_software_list(connection, software_list.id){
                    Ok(software_list) => {
                        println!("{:?}", software_list);
                    },
                    Err(e) => println!("Error fetching software list: {}", e),
                }
            }
        },
        Err(e) => println!("Error inserting software list: {}", e),
    }

}