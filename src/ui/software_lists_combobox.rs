use crate::models::SoftwareList;
use eframe::egui;

#[derive(Clone)]
pub struct SoftwareListSelectionOptions {
    pub selected_software_list_id: i32,
    pub software_lists: Vec<SoftwareList>,
}

pub struct SoftwareListsComboBox<'a> {
    ui: &'a mut egui::Ui,
    software_list_selection_options: SoftwareListSelectionOptions,
    on_software_list_id_changed: &'a mut dyn FnMut(i32),
}

impl<'a> SoftwareListsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        software_list_selection_options: SoftwareListSelectionOptions,
        on_software_list_id_changed: &'a mut dyn FnMut(i32),
    ) -> Self {
        Self {
            ui,
            software_list_selection_options,
            on_software_list_id_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Software Lists")
            .selected_text(
                &self
                    .software_list_selection_options
                    .software_lists
                    .iter()
                    .find(|s| {
                        s.id == self
                            .software_list_selection_options
                            .selected_software_list_id
                    })
                    .map(|s| s.name.clone())
                    .unwrap_or_default(),
            )
            .show_ui(self.ui, |ui| {
                let mut selected_software_list_id = self
                    .software_list_selection_options
                    .selected_software_list_id
                    .clone();
                for software_list in self.software_list_selection_options.software_lists.iter() {
                    if ui
                        .selectable_value(
                            &mut selected_software_list_id,
                            software_list.id.clone(),
                            software_list.name.clone(),
                        )
                        .clicked()
                    {
                        (self.on_software_list_id_changed)(selected_software_list_id);
                    }
                }
            });
    }
}

pub fn show_software_lists_combobox(
    ui: &mut egui::Ui,
    software_list_selection_options: SoftwareListSelectionOptions,
    on_software_list_id_changed: &mut dyn FnMut(i32),
) {
    let mut combobox = SoftwareListsComboBox::new(
        ui,
        software_list_selection_options,
        on_software_list_id_changed,
    );
    combobox.show();
}
