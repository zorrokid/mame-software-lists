use diesel::prelude::*;
use crate::models::System;

pub fn db_get_system(conn: &mut SqliteConnection, name_input: String) -> Result<Option<System>, diesel::result::Error> {
    use crate::schema::systems::dsl::*;
    systems.filter(name.eq(name_input))
        .first(conn)
        .optional()
}

pub fn db_get_systems(conn: &mut SqliteConnection) -> Result<Vec<System>, diesel::result::Error> {
    use crate::schema::systems::dsl::*;
    systems.load::<System>(conn)
}
