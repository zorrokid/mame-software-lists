use diesel::prelude::*;
use crate::models;

pub fn get_machines(conn: &mut SqliteConnection, input_software_list_id: &i32) -> Result<Vec<models::Machine>, diesel::result::Error> {
    use crate::schema::machine::dsl::*;
    machine.filter(software_list_id.eq(input_software_list_id))
        .load::<models::Machine>(conn)

}