use crate::configuration::emulators::Emulator;
use eframe::egui;

#[derive(Clone)]
pub struct EmulatorSelectionOptions {
    pub selected_emulator_id: String,
    pub emulators: Vec<Emulator>,
}
struct EmulatorsCombobox<'a> {
    ui: &'a mut egui::Ui,
    emulator_selection_options: EmulatorSelectionOptions,
    on_emulator_id_changed: &'a mut dyn FnMut(String),
}

impl<'a> EmulatorsCombobox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        emulator_selection_options: EmulatorSelectionOptions,
        on_emulator_id_changed: &'a mut dyn FnMut(String),
    ) -> Self {
        Self {
            ui,
            emulator_selection_options,
            on_emulator_id_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Emulators")
            .selected_text(
                self.emulator_selection_options
                    .emulators
                    .iter()
                    .find(|s| s.id == self.emulator_selection_options.selected_emulator_id)
                    .map(|s| s.description.clone())
                    .unwrap_or_default(),
            )
            .show_ui(self.ui, |ui| {
                let selected_emulator_id =
                    &mut self.emulator_selection_options.selected_emulator_id;
                for emulator in self.emulator_selection_options.emulators.iter() {
                    if ui
                        .selectable_value(
                            selected_emulator_id,
                            emulator.id.clone(),
                            emulator.description.clone(),
                        )
                        .clicked()
                    {
                        (self.on_emulator_id_changed)(selected_emulator_id.clone());
                    }
                }
            });
    }
}

pub fn show_emulators_combobox(
    ui: &mut egui::Ui,
    emulator_selection_options: EmulatorSelectionOptions,
    on_emulator_id_changed: &mut dyn FnMut(String),
) {
    EmulatorsCombobox::new(ui, emulator_selection_options, on_emulator_id_changed).show();
}
