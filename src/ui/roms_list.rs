use crate::models::Rom;
use crate::ui::selection_options::SelectionOptions;
use eframe::egui;

pub struct RomsList<'a> {
    ui: &'a mut egui::Ui,
    rom_selection_options: &'a SelectionOptions<Rom>,
    on_rom_selected: &'a mut dyn FnMut(Option<Rom>),
}

impl<'a> RomsList<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        rom_selection_options: &'a SelectionOptions<Rom>,
        on_rom_selected: &'a mut dyn FnMut(Option<Rom>),
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
                ui.heading("Files");
                egui::Grid::new("roms_table").show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Available");
                    ui.end_row();

                    let roms = self.rom_selection_options.items.clone();
                    let mut selected_rom = self.rom_selection_options.selected.clone();

                    for rom in roms.iter() {
                        if ui
                            .selectable_value(
                                &mut selected_rom,
                                Some(rom.clone()),
                                rom.name.clone(),
                            )
                            .clicked()
                        {
                            (self.on_rom_selected)(selected_rom.clone());
                        }
                        ui.label(match rom.available {
                            None => "Unknown".to_string(),
                            Some(true) => "Yes".to_string(),
                            Some(false) => "No".to_string(),
                        });
                        ui.end_row();
                    }
                });
            });
    }
}
