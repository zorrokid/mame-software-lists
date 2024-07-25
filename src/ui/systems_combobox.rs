use crate::models::System;
use eframe::egui;

#[derive(Clone)]
pub struct SystemSelectionOptions {
    pub selected_system_id: i32,
    pub systems: Vec<System>,
}

impl SystemSelectionOptions {
    pub fn new(selected_system_id: i32, systems: Vec<System>) -> Self {
        Self {
            selected_system_id,
            systems,
        }
    }

    pub fn get_selected_system(&self) -> Option<&System> {
        self.systems
            .iter()
            .find(|s| s.id == self.selected_system_id)
    }
}

pub struct SystemsComboBox<'a> {
    ui: &'a mut egui::Ui,
    system_selection_options: SystemSelectionOptions,
    on_system_id_changed: &'a mut dyn FnMut(i32),
}

impl<'a> SystemsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        system_selection_options: SystemSelectionOptions,
        on_system_id_changed: &'a mut dyn FnMut(i32),
    ) -> Self {
        Self {
            ui,
            system_selection_options,
            on_system_id_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Systems")
            .selected_text(
                &self
                    .system_selection_options
                    .systems
                    .iter()
                    .find(|s| s.id == self.system_selection_options.selected_system_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default(),
            )
            .show_ui(self.ui, |ui| {
                let mut selected_system_id =
                    self.system_selection_options.selected_system_id.clone();
                for system in self.system_selection_options.systems.iter() {
                    if ui
                        .selectable_value(
                            &mut selected_system_id,
                            system.id.clone(),
                            system.name.clone(),
                        )
                        .clicked()
                    {
                        (self.on_system_id_changed)(selected_system_id);
                    }
                }
            });
    }
}

pub fn show_systems_combobox(
    ui: &mut egui::Ui,
    system_selection_options: SystemSelectionOptions,
    on_system_id_changed: &mut dyn FnMut(i32),
) {
    let mut combobox = SystemsComboBox::new(ui, system_selection_options, on_system_id_changed);
    combobox.show();
}
