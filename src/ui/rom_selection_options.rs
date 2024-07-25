use crate::models::Rom;
pub struct RomSelectionOptions {
    selected_rom_id: i32,
    previous_selected_rom_id: i32,
    new_selected_rom_id: Option<i32>,
    roms: Vec<Rom>,
}

impl RomSelectionOptions {
    pub fn new(
        selected_rom_id: i32,
        previous_selected_rom_id: i32,
        new_selected_rom_id: Option<i32>,
        roms: Vec<Rom>,
    ) -> Self {
        Self {
            selected_rom_id,
            previous_selected_rom_id,
            new_selected_rom_id,
            roms,
        }
    }
    pub fn get_selected_rom(&self) -> Option<&Rom> {
        self.roms.iter().find(|r| r.id == self.selected_rom_id)
    }

    pub fn get_selected_rom_id(&self) -> i32 {
        self.selected_rom_id
    }

    pub fn get_roms(&self) -> &Vec<Rom> {
        &self.roms
    }

    pub fn set_roms(&mut self, roms: Vec<Rom>) {
        self.roms = roms;
    }

    pub fn get_new_selected_rom_id(&self) -> Option<i32> {
        self.new_selected_rom_id
    }

    pub fn set_new_selected_rom_id(&mut self, new_selected_rom_id: Option<i32>) {
        self.new_selected_rom_id = new_selected_rom_id;
    }

    pub fn set_selected_rom_id(&mut self, selected_rom_id: i32) {
        self.selected_rom_id = selected_rom_id;
    }

    pub fn get_previous_selected_rom_id(&self) -> i32 {
        self.previous_selected_rom_id
    }

    pub fn set_previous_selected_rom_id(&mut self, previous_selected_rom_id: i32) {
        self.previous_selected_rom_id = previous_selected_rom_id;
    }
}
