use crate::models::Machine;
use eframe::egui;
pub struct MachinePanel<'a> {
    ui: &'a mut egui::Ui,
    selected_machine: &'a Option<Machine>,
}

impl<'a> MachinePanel<'a> {
    pub fn new(ui: &'a mut egui::Ui, selected_machine: &'a Option<Machine>) -> Self {
        Self {
            ui,
            selected_machine,
        }
    }

    pub fn show(&mut self) {
        egui::ScrollArea::vertical()
            .id_source("machine_panel_scroll_area")
            .show(&mut self.ui, |ui| {
                let selected_machine = self.selected_machine.clone().unwrap_or_default();
                ui.heading(&selected_machine.description);
                ui.label(format!(
                    "Year: {}",
                    &selected_machine.year.unwrap_or_default()
                ));
                ui.label(format!("Publisher: {}", &selected_machine.publisher));
            });
    }
}
