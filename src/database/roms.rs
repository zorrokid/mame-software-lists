use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;

pub fn db_get_roms(
    conn: &mut SqliteConnection,
    machine: &Machine,
) -> Result<Vec<Rom>, diesel::result::Error> {
    MachineRom::belonging_to(&machine)
        .inner_join(roms::table)
        .select(Rom::as_select())
        .load(conn)
}

pub fn update(conn: &mut SqliteConnection, roms: &Vec<Rom>) -> Result<(), diesel::result::Error> {
    for rom in roms {
        diesel::update(roms::table.find(rom.id))
            .set(rom)
            .execute(conn)?;
    }
    Ok(())
}
