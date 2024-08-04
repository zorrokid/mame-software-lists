use eframe::egui;

use super::{
    combobox::ComboBox, machine_panel::MachinePanel, machines_list::MachinesList,
    message_dialog::MessageDialog, roms_list::RomsList,
    software_list_selection_dialog::SoftwareListSelectionDialog, ui_state::UiState,
};

use crate::configuration::emulators::Emulator;
use crate::models::{SoftwareList, System};

pub struct MameSoftwareListApp {
    ui_state: UiState,
}

impl MameSoftwareListApp {
    pub fn new() -> Self {
        Self {
            ui_state: UiState::new(),
        }
    }
}

impl eframe::App for MameSoftwareListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Add Software Lists data file").clicked() {
                        self.ui_state.add_software_list_data_file();
                    }
                    if ui
                        .button("Scan available files for a software list")
                        .clicked()
                    {
                        self.ui_state.scan_available_files();
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let label = "Systems".to_string();
                ComboBox::<System>::new(
                    ui,
                    &self.ui_state.system_selection_options.clone(),
                    &mut |system| self.ui_state.on_system_changed(system),
                    &label,
                )
                .show();

                let label = "Software lists".to_string();
                ComboBox::<SoftwareList>::new(
                    ui,
                    &self.ui_state.software_list_selection_options.clone(),
                    &mut |software_list| {
                        self.ui_state
                            .on_software_list_selection_changed(software_list);
                    },
                    &label,
                )
                .show();

                let label = "Emulators".to_string();
                ComboBox::<Emulator>::new(
                    ui,
                    &self.ui_state.emulator_selection_options.clone(),
                    &mut |emulator| {
                        self.ui_state.on_emulator_changed(emulator);
                    },
                    &label,
                )
                .show();

                if ui.button("Start").clicked() {
                    self.ui_state.start_button_clicked();
                }
            });

            ui.add_sized(ui.available_size(), |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.centered_and_justified(|ui| {
                        MachinesList::new(
                            ui,
                            &self.ui_state.machine_selection_options.clone(),
                            &mut |machine_id| {
                                self.ui_state.on_machine_selection_changed(machine_id)
                            },
                        )
                        .show();
                    });
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        MachinePanel::new(ui, &self.ui_state.machine_selection_options.selected)
                            .show();
                        RomsList::new(
                            ui,
                            &self.ui_state.rom_selection_options.clone(),
                            &mut |rom_id| self.ui_state.on_rom_selected(rom_id),
                        )
                        .show();
                    });
                })
                .response
            });

            if self.ui_state.software_list_selection_dialog_options.show {
                let software_lists = self.ui_state.get_all_software_lists();
                SoftwareListSelectionDialog::new(
                    &mut |software_list: Option<&SoftwareList>| {
                        self.ui_state
                            .close_software_list_selection_dialog(software_list)
                    },
                    &software_lists,
                )
                .show(ctx);
            }

            if self.ui_state.message_dialog_options.show {
                let message_dialog_options = self.ui_state.message_dialog_options.message.clone();
                MessageDialog::new(ctx, &message_dialog_options, &mut || {
                    self.ui_state.close_message_dialog();
                })
                .show();
            }

            self.ui_state.on_update();
        });

        egui::TopBottomPanel::bottom("message_console")
            .exact_height(80.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let reverse_messages = self.ui_state.console_messages.iter().rev();
                    for message in reverse_messages {
                        ui.label(message);
                    }
                });
            });
    }
}
