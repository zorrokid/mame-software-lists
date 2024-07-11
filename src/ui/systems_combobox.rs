use crate::models::System;
use eframe::egui;

pub struct SystemsComboBox<'a> {
    ui: &'a mut egui::Ui,
    systems: &'a Vec<System>,
    selected_system_id: &'a mut i32,
    previous_selected_system_id: &'a mut i32,
    new_selected_systemid: &'a mut Option<i32>,
}

impl<'a> SystemsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        systems: &'a Vec<System>,
        selected_system_id: &'a mut i32,
        previous_selected_system_id: &'a mut i32,
        new_selected_systemid: &'a mut Option<i32>,
    ) -> Self {
        Self {
            ui,
            systems,
            selected_system_id,
            previous_selected_system_id,
            new_selected_systemid,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Systems")
            .selected_text(
                &self
                    .systems
                    .iter()
                    .find(|s| s.id == *self.selected_system_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default(),
            )
            .show_ui(self.ui, |ui| {
                for system in self.systems.iter() {
                    if ui
                        .selectable_value(
                            self.selected_system_id,
                            system.id.clone(),
                            system.name.clone(),
                        )
                        .clicked()
                    {
                        if *self.selected_system_id != *self.previous_selected_system_id {
                            *self.new_selected_systemid = Some(self.selected_system_id.clone());
                        }
                    }
                }
            });
    }
}

pub fn show_systems_combobox(
    ui: &mut egui::Ui,
    systems: &Vec<System>,
    selected_system_id: &mut i32,
    previous_selected_system_id: &mut i32,
    new_selected_systemid: &mut Option<i32>,
) {
    let mut combobox = SystemsComboBox::new(
        ui,
        systems,
        selected_system_id,
        previous_selected_system_id,
        new_selected_systemid,
    );
    combobox.show();
}
