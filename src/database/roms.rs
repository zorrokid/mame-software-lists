use diesel::prelude::*;
use crate::models::*;
use crate::schema::*;

pub fn db_get_roms(conn: &mut SqliteConnection, machine: &Machine) -> Result<Vec<Rom>, diesel::result::Error> {
    MachineRom::belonging_to(&machine)
        .inner_join(roms::table)
        .select(Rom::as_select())
        .load(conn)
}

pub fn set_matched_roms(conn: &mut SqliteConnection, matched_rom_ids: &Vec<i32>) {
        for id in matched_rom_ids {
            diesel::update(roms::table.find(id))
                .set(roms::columns::have.eq(true))
                .execute(conn)
                .expect("Error updating roms");
    }
}