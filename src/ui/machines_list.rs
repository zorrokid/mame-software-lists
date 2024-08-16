use crate::models::Machine;
use crate::ui::selection_options::SelectionOptions;
use eframe::egui;

const MAX_NAME_LENGTH: usize = 50;
const MAX_ITEMS_PER_PAGE: usize = 50;
pub type MachineSelectionOptions = SelectionOptions<Machine>;

pub struct MachinesList {
    first_visible_item_index: usize,
}

impl<'a> MachinesList {
    pub fn new() -> Self {
        Self {
            first_visible_item_index: 0,
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        machine_selection_options: &'a MachineSelectionOptions,
        on_machine_selected: &'a mut dyn FnMut(Option<Machine>),
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("Up").clicked() {
                    self.first_visible_item_index = self
                        .first_visible_item_index
                        .saturating_sub(MAX_ITEMS_PER_PAGE);
                }
                if ui.button("Down").clicked() {
                    if self.first_visible_item_index + MAX_ITEMS_PER_PAGE
                        < machine_selection_options.items.len()
                    {
                        self.first_visible_item_index += MAX_ITEMS_PER_PAGE;
                    }
                }
            });
            egui::ScrollArea::vertical()
                .id_source("machines_scroll_area")
                .show(ui, |ui| {
                    egui::Grid::new("machines_table").show(ui, |ui| {
                        ui.label("Name");
                        ui.label("Year");
                        ui.end_row();

                        let mut selected_machine = machine_selection_options.selected.clone();
                        let visible_machines = machine_selection_options
                            .items
                            .iter()
                            .skip(self.first_visible_item_index)
                            .take(MAX_ITEMS_PER_PAGE);
                        for machine in visible_machines {
                            let truncated_name = if machine.description.len() > MAX_NAME_LENGTH {
                                format!(
                                    "{}...",
                                    &machine
                                        .description
                                        .chars()
                                        .take(MAX_NAME_LENGTH)
                                        .collect::<String>()
                                )
                            } else {
                                machine.description.clone()
                            };
                            if ui
                                .selectable_value(
                                    &mut selected_machine,
                                    Some(machine.clone()),
                                    truncated_name,
                                )
                                .clicked()
                            {
                                (on_machine_selected)(selected_machine.clone());
                            }
                            ui.label(&machine.year.unwrap_or_default().to_string());
                            ui.end_row();
                        }
                    });
                });
        });
    }
}
