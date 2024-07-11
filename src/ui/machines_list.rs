use crate::models::Machine;
use eframe::egui;

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
        egui::ScrollArea::vertical()
            .id_source("machines_scroll_area")
            .show(self.ui, |ui| {
                egui::Grid::new("machines_table").show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Year");
                    ui.end_row();

                    for machine in self.machines.iter() {
                        if ui
                            .selectable_value(
                                self.selected_machine_id,
                                machine.id.clone(),
                                machine.description.clone(),
                            )
                            .clicked()
                        {
                            if *self.selected_machine_id != *self.previous_selected_machine_id {
                                *self.new_selected_machine_id = Some(self.selected_machine_id.clone());
                            }
                        }
                        ui.label(&machine.year.unwrap_or_default().to_string());
                        ui.end_row();
                    }
                });
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
    )
    .show();
}
