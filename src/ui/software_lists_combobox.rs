use crate::models::SoftwareList;
use eframe::egui;

pub type SoftwareListSelectionOptions =
    crate::ui::selection_options::SelectionOptions<SoftwareList>;

pub struct SoftwareListsComboBox<'a> {
    ui: &'a mut egui::Ui,
    software_list_selection_options: SoftwareListSelectionOptions,
    on_selected_software_list_changed: &'a mut dyn FnMut(Option<SoftwareList>),
}

impl<'a> SoftwareListsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        software_list_selection_options: SoftwareListSelectionOptions,
        on_selected_software_list_changed: &'a mut dyn FnMut(Option<SoftwareList>),
    ) -> Self {
        Self {
            ui,
            software_list_selection_options,
            on_selected_software_list_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Software Lists")
            .selected_text(
                &self
                    .software_list_selection_options
                    .selected
                    .clone()
                    .map(|s| s.name)
                    .unwrap_or("".to_string()),
            )
            .show_ui(self.ui, |ui| {
                let mut selected_software_list =
                    self.software_list_selection_options.selected.clone();
                for software_list in self.software_list_selection_options.items.iter() {
                    if ui
                        .selectable_value(
                            &mut selected_software_list,
                            Some(software_list.clone()),
                            software_list.name.clone(),
                        )
                        .clicked()
                    {
                        (self.on_selected_software_list_changed)(selected_software_list.clone());
                    }
                }
            });
    }
}

pub fn show_software_lists_combobox(
    ui: &mut egui::Ui,
    software_list_selection_options: SoftwareListSelectionOptions,
    on_selected_software_list_changed: &mut dyn FnMut(Option<SoftwareList>),
) {
    let mut combobox = SoftwareListsComboBox::new(
        ui,
        software_list_selection_options,
        on_selected_software_list_changed,
    );
    combobox.show();
}
