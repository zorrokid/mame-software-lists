use eframe::egui;
use crate::models::SoftwareList;

pub struct SoftwareListsComboBox<'a> {
    ui: &'a mut egui::Ui,
    software_lists: &'a Vec<SoftwareList>,
    selected_software_list_id: &'a mut i32,
    previous_selected_software_list_id: &'a mut i32,
    new_selected_software_list_id: &'a mut Option<i32>,
}

impl<'a> SoftwareListsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        software_lists: &'a Vec<SoftwareList>,
        selected_software_list_id: &'a mut i32,
        previous_selected_software_list_id: &'a mut i32,
        new_selected_software_list_id: &'a mut Option<i32>,
    ) -> Self {
        Self {
            ui,
            software_lists,
            selected_software_list_id,
            previous_selected_software_list_id,
            new_selected_software_list_id,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Software Lists")
            .selected_text(
                &self.software_lists
                    .iter()
                    .find(|s| s.id == *self.selected_software_list_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default()
            )
            .show_ui(self.ui, |ui| {
                for software_list in self.software_lists.iter() {
                    if ui.selectable_value(
                        self.selected_software_list_id, 
                        software_list.id.clone(), 
                        software_list.name.clone()
                    ).clicked() {
                        if *self.selected_software_list_id != *self.previous_selected_software_list_id{
                            *self.new_selected_software_list_id= Some(self.selected_software_list_id.clone());
                        }
                    }
                }
            });
    }

}

pub fn show_software_lists_combobox(
    ui: &mut egui::Ui,
    software_lists: &Vec<SoftwareList>,
    selected_software_list_id: &mut i32,
    previous_selected_software_list_id: &mut i32,
    new_selected_software_list_id: &mut Option<i32>,
) {
    
    let mut combobox = SoftwareListsComboBox::new(
        ui, 
        software_lists, 
        selected_software_list_id, 
        previous_selected_software_list_id, 
        new_selected_software_list_id
    );
    combobox.show();
}
