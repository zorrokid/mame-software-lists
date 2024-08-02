use eframe::egui;

use super::{
    combobox::{
        emulators_combobox::show_emulators_combobox,
        software_lists_combobox::show_software_lists_combobox,
        systems_combobox::show_systems_combobox,
    },
    machine_panel::show_machine_panel,
    machines_list::show_machines_list,
    message_dialog::show_message_dialog,
    roms_list::show_roms_list,
    scan_files_dialog::show_scan_files_dialog,
    ui_state::UiState,
};

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
                show_systems_combobox(
                    ui,
                    self.ui_state.system_selection_options.clone(),
                    &mut |id| self.ui_state.on_system_changed(id),
                );

                show_software_lists_combobox(
                    ui,
                    self.ui_state.software_list_selection_options.clone(),
                    &mut |id| {
                        self.ui_state.on_software_list_selection_changed(id);
                    },
                );

                show_emulators_combobox(
                    ui,
                    self.ui_state.emulator_selection_options.clone(),
                    &mut |id| {
                        self.ui_state.on_emulator_id_changed(id);
                    },
                );

                if ui.button("Start").clicked() {
                    self.ui_state.start_button_clicked();
                }
            });

            ui.add_sized(ui.available_size(), |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.centered_and_justified(|ui| {
                        show_machines_list(
                            ui,
                            self.ui_state.machine_selection_options.clone(),
                            &mut |machine_id| {
                                self.ui_state.on_machine_selection_changed(machine_id)
                            },
                        );
                    });
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        show_machine_panel(ui, &self.ui_state.machine_selection_options.selected);
                        show_roms_list(
                            ui,
                            &self.ui_state.rom_selection_options.clone(),
                            &mut |rom_id| self.ui_state.on_rom_selected(rom_id),
                        );
                    });
                })
                .response
            });

            if self.ui_state.scan_files_dialog_options.show {
                let cloned_software_lists = self
                    .ui_state
                    .scan_files_dialog_options
                    .software_lists
                    .clone();
                let selected_software_list_id = self
                    .ui_state
                    .scan_files_dialog_options
                    .selected_software_list_id;
                show_scan_files_dialog(
                    ctx,
                    |id: Option<i32>| self.ui_state.close_available_files_dialog(id),
                    &cloned_software_lists,
                    selected_software_list_id,
                );
            }

            if self.ui_state.message_dialog_options.show {
                show_message_dialog(
                    ctx,
                    &self.ui_state.message_dialog_options.message,
                    &mut || {
                        self.ui_state.message_dialog_options.show = false;
                    },
                )
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
