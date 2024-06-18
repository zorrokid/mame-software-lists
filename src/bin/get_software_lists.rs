use mame_software_lists::database::establish_connection;
use mame_software_lists::database::software_lists::get_software_lists;
use mame_software_lists::database::machines::get_machines;

fn main() {
    let connection = &mut establish_connection();
    match get_software_lists(connection){
        Ok(software_lists) => {
            if software_lists.len() == 0 {
                println!("No software lists in database.");
            }
            for software_list in software_lists {
                println!("{:?}", software_list);
                match get_machines(connection, &software_list.id.unwrap()) {
                    Ok(machines) => {
                        if machines.len() == 0 {
                            println!("No machines in software list.");
                        }
                        for machine in machines {
                            println!("{:?}", machine);
                        }
                    },
                    Err(e) => println!("Error getting machines: {}", e),
                }
            }
        },
        Err(e) => println!("Error inserting software list: {}", e),
    }

}