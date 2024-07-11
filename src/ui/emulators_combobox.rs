use crate::configuration::emulators::Emulator;
use eframe::egui;

struct EmulatorsCombobox<'a> {
    ui: &'a mut egui::Ui,
    emulators: &'a Vec<Emulator>,
    selected_emulator_id: &'a mut String,
}

impl<'a> EmulatorsCombobox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        emulators: &'a Vec<Emulator>,
        selected_emulator_id: &'a mut String,
    ) -> Self {
        Self {
            ui,
            emulators,
            selected_emulator_id,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Emulators")
            .selected_text(
                self.emulators
                    .iter()
                    .find(|s| s.id == *self.selected_emulator_id)
                    .map(|s| s.description.clone())
                    .unwrap_or_default(),
            )
            .show_ui(self.ui, |ui| {
                for emulator in self.emulators.iter() {
                    if ui
                        .selectable_value(
                            self.selected_emulator_id,
                            emulator.id.clone(),
                            emulator.description.clone(),
                        )
                        .clicked()
                    {}
                }
            });
    }
}

pub fn show_emulators_combobox(
    ui: &mut egui::Ui,
    emulators: &Vec<Emulator>,
    selected_emulator_id: &mut String,
) {
    EmulatorsCombobox::new(ui, emulators, selected_emulator_id).show();
}
