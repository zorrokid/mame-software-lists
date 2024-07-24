use crate::models::Rom;
use eframe::egui;

pub struct RomsList<'a> {
    ui: &'a mut egui::Ui,
    roms: &'a Vec<Rom>,
    selected_rom_id: &'a mut i32,
    previous_selected_rom_id: &'a mut i32,
    new_selected_rom_id: &'a mut Option<i32>,
}

impl<'a> RomsList<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        roms: &'a Vec<Rom>,
        selected_rom_id: &'a mut i32,
        previous_selected_rom_id: &'a mut i32,
        new_selected_rom_id: &'a mut Option<i32>,
    ) -> Self {
        Self {
            ui,
            roms,
            selected_rom_id,
            previous_selected_rom_id,
            new_selected_rom_id,
        }
    }

    pub fn show(&mut self) {
        egui::ScrollArea::vertical()
            .id_source("roms_scroll_area")
            .show(self.ui, |ui| {
                egui::Grid::new("roms_table").show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Available");
                    ui.end_row();

                    for rom in self.roms.iter() {
                        if ui
                            .selectable_value(
                                self.selected_rom_id,
                                rom.id.clone(),
                                rom.name.clone(),
                            )
                            .clicked()
                        {
                            if *self.selected_rom_id != *self.previous_selected_rom_id {
                                *self.new_selected_rom_id = Some(self.selected_rom_id.clone());
                            }
                        }
                        ui.label(if rom.have { "Yes" } else { "No" });
                        ui.end_row();
                    }
                });
            });
    }
}

pub fn show_roms_list(
    ui: &mut egui::Ui,
    roms: &Vec<Rom>,
    selected_rom_id: &mut i32,
    previous_selected_rom_id: &mut i32,
    new_selected_rom_id: &mut Option<i32>,
) {
    RomsList::new(
        ui,
        roms,
        selected_rom_id,
        previous_selected_rom_id,
        new_selected_rom_id,
    )
    .show();
}
