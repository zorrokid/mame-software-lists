use crate::configuration::emulators::Emulator;
use crate::ui::combobox::ComboBox;
use eframe::egui;

pub type EmulatorSelectionOptions = crate::ui::selection_options::SelectionOptions<Emulator>;

pub fn show_emulators_combobox(
    ui: &mut egui::Ui,
    selection_options: EmulatorSelectionOptions,
    on_selected_changed: &mut dyn FnMut(Option<Emulator>),
) {
    let label = "Emulators".to_string();
    let mut combobox =
        ComboBox::<Emulator>::new(ui, selection_options, on_selected_changed, &label);
    combobox.show();
}
