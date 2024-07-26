use crate::configuration::emulators::Emulator;
use eframe::egui;

#[derive(Clone)]
pub struct EmulatorSelectionOptions {
    pub selected_emulator: Option<Emulator>,
    pub emulators: Vec<Emulator>,
}
struct EmulatorsCombobox<'a> {
    ui: &'a mut egui::Ui,
    emulator_selection_options: EmulatorSelectionOptions,
    on_selected_emulator_changed: &'a mut dyn FnMut(Option<Emulator>),
}

impl<'a> EmulatorsCombobox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        emulator_selection_options: EmulatorSelectionOptions,
        on_selected_emulator_changed: &'a mut dyn FnMut(Option<Emulator>),
    ) -> Self {
        Self {
            ui,
            emulator_selection_options,
            on_selected_emulator_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Emulators")
            .selected_text(
                self.emulator_selection_options
                    .selected_emulator
                    .clone()
                    .map(|s| s.description)
                    .unwrap_or("".to_string()),
            )
            .show_ui(self.ui, |ui| {
                let selected_emulator = &mut self.emulator_selection_options.selected_emulator;
                for emulator in self.emulator_selection_options.emulators.iter() {
                    if ui
                        .selectable_value(
                            selected_emulator,
                            Some(emulator.clone()),
                            emulator.description.clone(),
                        )
                        .clicked()
                    {
                        (self.on_selected_emulator_changed)(selected_emulator.clone());
                    }
                }
            });
    }
}

pub fn show_emulators_combobox(
    ui: &mut egui::Ui,
    emulator_selection_options: EmulatorSelectionOptions,
    on_selected_emulator_changed: &mut dyn FnMut(Option<Emulator>),
) {
    EmulatorsCombobox::new(ui, emulator_selection_options, on_selected_emulator_changed).show();
}
