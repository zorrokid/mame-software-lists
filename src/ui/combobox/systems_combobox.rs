use super::combobox::ComboBox;
use crate::models::System;
use crate::ui::selection_options::SelectionOptions;
use eframe::egui;

pub type SystemSelectionOptions = SelectionOptions<System>;

pub fn show_systems_combobox(
    ui: &mut egui::Ui,
    selection_options: SelectionOptions<System>,
    on_selected_changed: &mut dyn FnMut(Option<System>),
) {
    let label = "Systems".to_string();
    let mut combobox = ComboBox::<System>::new(ui, selection_options, on_selected_changed, &label);
    combobox.show();
}
