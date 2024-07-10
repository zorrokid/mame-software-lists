use eframe::egui;
use crate::models::Machine;

pub struct MachinesList<'a> {
    ui: &'a mut egui::Ui,
    machines: &'a Vec<Machine>,
    selected_machine_id: &'a mut i32,
    previous_selected_machine_id: &'a mut i32,
    new_selected_machine_id: &'a mut Option<i32>,
}

impl<'a> MachinesList<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        machines: &'a Vec<Machine>,
        selected_machine_id: &'a mut i32,
        previous_selected_machine_id: &'a mut i32,
        new_selected_machine_id: &'a mut Option<i32>,
    ) -> Self {
        Self {
            ui,
            machines,
            selected_machine_id,
            previous_selected_machine_id,
            new_selected_machine_id,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Machines")
            .selected_text(
                &self.machines
                    .iter()
                    .find(|s| s.id == *self.selected_machine_id)
                    .map(|s| s.description.clone())
                    .unwrap_or_default()
            )
            .show_ui(self.ui, |ui| {
                for machine in self.machines.iter() {
                    if ui.selectable_value(
                        self.selected_machine_id, 
                        machine.id.clone(), 
                        machine.description.clone()
                    ).clicked() {
                        if *self.selected_machine_id != *self.previous_selected_machine_id {
                            *self.new_selected_machine_id = Some(self.selected_machine_id.clone());
                        }
                    }
                }
            });
    }   
}

pub fn show_machines_list(
    ui: &mut egui::Ui,
    machines: &Vec<Machine>,
    selected_machine_id: &mut i32,
    previous_selected_machine_id: &mut i32,
    new_selected_machine_id: &mut Option<i32>,
) {
    MachinesList::new(
        ui,
        machines,
        selected_machine_id,
        previous_selected_machine_id,
        new_selected_machine_id,
    ).show();
}