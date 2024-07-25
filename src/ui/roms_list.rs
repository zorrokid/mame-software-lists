use crate::models::Rom;
use eframe::egui;

#[derive(Clone)]
pub struct RomSelectionOptions {
    pub selected_rom_id: i32,
    pub roms: Vec<Rom>,
}

impl RomSelectionOptions {
    pub fn new(selected_rom_id: i32, roms: Vec<Rom>) -> Self {
        Self {
            selected_rom_id,
            roms,
        }
    }
    pub fn get_selected_rom(&self) -> Option<&Rom> {
        self.roms.iter().find(|r| r.id == self.selected_rom_id)
    }
}

pub struct RomsList<'a> {
    ui: &'a mut egui::Ui,
    rom_selection_options: &'a RomSelectionOptions,
    on_rom_selected: &'a mut dyn FnMut(i32),
}

impl<'a> RomsList<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        rom_selection_options: &'a RomSelectionOptions,
        on_rom_selected: &'a mut dyn FnMut(i32),
    ) -> Self {
        Self {
            ui,
            rom_selection_options,
            on_rom_selected,
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

                    let roms = self.rom_selection_options.roms.clone();
                    let mut selected_rom_id = self.rom_selection_options.selected_rom_id.clone();

                    for rom in roms.iter() {
                        if ui
                            .selectable_value(
                                &mut selected_rom_id,
                                rom.id.clone(),
                                rom.name.clone(),
                            )
                            .clicked()
                        {
                            (self.on_rom_selected)(selected_rom_id);
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
    rom_selection_options: &RomSelectionOptions,
    on_rom_selected: &mut dyn FnMut(i32),
) {
    RomsList::new(ui, rom_selection_options, on_rom_selected).show();
}
