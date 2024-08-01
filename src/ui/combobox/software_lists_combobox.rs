use super::combobox::ComboBox;
use crate::models::SoftwareList;
use eframe::egui;

pub type SoftwareListSelectionOptions =
    crate::ui::selection_options::SelectionOptions<SoftwareList>;

pub fn show_software_lists_combobox(
    ui: &mut egui::Ui,
    selection_options: SoftwareListSelectionOptions,
    on_selected_changed: &mut dyn FnMut(Option<SoftwareList>),
) {
    let label = "Software lists".to_string();
    let mut combobox =
        ComboBox::<SoftwareList>::new(ui, selection_options, on_selected_changed, &label);
    combobox.show();
}
