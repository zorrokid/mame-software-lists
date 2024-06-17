use mame_software_lists::database::establish_connection;
use mame_software_lists::database::get_software_lists;

fn main() {
    let connection = &mut establish_connection();
    match get_software_lists(connection){
        Ok(software_lists) => {
            // iterate through software lists and print them
            for software_list in software_lists {
                println!("{:?}", software_list);
            }
        },
        Err(e) => println!("Error inserting software list: {}", e),
    }

}