use eframe::egui;

use super::rom_selection_options::RomSelectionOptions;

pub struct RomsList<'a> {
    ui: &'a mut egui::Ui,
    rom_selection_options: &'a mut RomSelectionOptions,
}

impl<'a> RomsList<'a> {
    pub fn new(ui: &'a mut egui::Ui, rom_selection_options: &'a mut RomSelectionOptions) -> Self {
        Self {
            ui,
            rom_selection_options,
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

                    let roms = self.rom_selection_options.get_roms().clone();
                    let mut selected_rom_id =
                        self.rom_selection_options.get_selected_rom_id().clone();

                    for rom in roms.iter() {
                        if ui
                            .selectable_value(
                                &mut selected_rom_id,
                                rom.id.clone(),
                                rom.name.clone(),
                            )
                            .clicked()
                        {
                            self.rom_selection_options
                                .set_selected_rom_id(selected_rom_id);
                        }
                        ui.label(if rom.have { "Yes" } else { "No" });
                        ui.end_row();
                    }
                });
            });
    }
}

pub fn show_roms_list(ui: &mut egui::Ui, rom_selection_options: &mut RomSelectionOptions) {
    RomsList::new(ui, rom_selection_options).show();
}
