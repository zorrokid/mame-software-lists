use crate::models::System;
use crate::ui::selection_options::SelectionOptions;
use eframe::egui;

pub type SystemSelectionOptions = SelectionOptions<System>;

pub struct SystemsComboBox<'a> {
    ui: &'a mut egui::Ui,
    system_selection_options: SystemSelectionOptions,
    on_selected_system_changed: &'a mut dyn FnMut(Option<System>),
}

impl<'a> SystemsComboBox<'a> {
    pub fn new(
        ui: &'a mut egui::Ui,
        system_selection_options: SystemSelectionOptions,
        on_selected_system_changed: &'a mut dyn FnMut(Option<System>),
    ) -> Self {
        Self {
            ui,
            system_selection_options,
            on_selected_system_changed,
        }
    }

    pub fn show(&mut self) {
        egui::ComboBox::from_label("Systems")
            .selected_text(
                &self
                    .system_selection_options
                    .selected
                    .clone()
                    .map(|s| s.name)
                    .unwrap_or("".to_string()),
            )
            .show_ui(self.ui, |ui| {
                let mut selected_system = self.system_selection_options.selected.clone();
                for system in self.system_selection_options.items.iter() {
                    if ui
                        .selectable_value(
                            &mut selected_system,
                            Some(system.clone()),
                            system.name.clone(),
                        )
                        .clicked()
                    {
                        (self.on_selected_system_changed)(selected_system.clone());
                    }
                }
            });
    }
}

pub fn show_systems_combobox(
    ui: &mut egui::Ui,
    system_selection_options: SystemSelectionOptions,
    on_selected_system_changed: &mut dyn FnMut(Option<System>),
) {
    let mut combobox =
        SystemsComboBox::new(ui, system_selection_options, on_selected_system_changed);
    combobox.show();
}
