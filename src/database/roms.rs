use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub fn db_get_roms(conn: &mut SqliteConnection, machine: &Machine) -> Result<Vec<Rom>, diesel::result::Error> {
    MachineRom::belonging_to(&machine)
        .inner_join(roms::table)
        .select(Rom::as_select())
        .load(conn)
}