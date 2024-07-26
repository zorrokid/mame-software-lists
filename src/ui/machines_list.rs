use crate::models::Machine;
use eframe::egui;

#[derive(Clone)]
pub struct MachineSelectionOptions {
    pub selected_machine: Option<Machine>,
    pub machines: Vec<Machine>,
}

pub struct MachinesList<'a> {
    ui: &'a mut egui::Ui,
    machine_selection_options: MachineSelectionOptions,
    on_machine_selected: &'a mut dyn FnMut(Option<Machine>),
}

impl<'a> MachinesList<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        machine_selection_options: MachineSelectionOptions,
        on_machine_selected: &'a mut dyn FnMut(Option<Machine>),
    ) -> Self {
        Self {
            ui,
            machine_selection_options,
            on_machine_selected,
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

                    let mut selected_machine =
                        self.machine_selection_options.selected_machine.clone();
                    for machine in self.machine_selection_options.machines.iter() {
                        if ui
                            .selectable_value(
                                &mut selected_machine,
                                Some(machine.clone()),
                                machine.description.clone(),
                            )
                            .clicked()
                        {
                            (self.on_machine_selected)(selected_machine.clone());
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
    machine_selection_options: MachineSelectionOptions,
    on_machine_selected: &mut dyn FnMut(Option<Machine>),
) {
    MachinesList::new(ui, machine_selection_options, on_machine_selected).show();
}
