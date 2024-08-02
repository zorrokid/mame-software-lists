use crate::{
    configuration::paths::Paths,
    emulators::emulator_runner::run_with_emulator,
    software_lists::software_list_file_scanner::{
        SoftwareListFileScanner, SoftwareListScannerError, SoftwareListScannerResult,
    },
};
use eframe::egui;
use rfd::FileDialog;
use std::{path::PathBuf, sync::mpsc, thread};

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
    paths: Paths,
    file_dialog_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
    software_list_file_scanner_receiver:
        Option<mpsc::Receiver<Option<Result<SoftwareListScannerResult, SoftwareListScannerError>>>>,
    ui_state: UiState,
}

impl MameSoftwareListApp {
    pub fn new(paths: Paths) -> Self {
        Self {
            ui_state: UiState::new(),
            paths,
            file_dialog_receiver: None,
            software_list_file_scanner_receiver: None,
        }
    }

    fn start_button_clicked(&mut self) {
        if self.ui_state.system_selection_options.selected.is_none() {
            self.ui_state
                .add_message("Please select a system".to_string());
            return;
        }
        if self.ui_state.machine_selection_options.selected.is_none() {
            self.ui_state
                .add_message("Please select a machine".to_string());
            return;
        }
        if self.ui_state.emulator_selection_options.selected.is_none() {
            self.ui_state
                .add_message("Please select an emulator".to_string());
            return;
        }
        let system_name = self
            .ui_state
            .system_selection_options
            .selected
            .clone()
            .unwrap()
            .name;
        let machine = self
            .ui_state
            .machine_selection_options
            .selected
            .clone()
            .unwrap();
        let emulator = self
            .ui_state
            .emulator_selection_options
            .selected
            .clone()
            .unwrap();
        let rom = self.ui_state.rom_selection_options.selected.clone();
        let paths = self.paths.clone();

        self.ui_state.add_console_message(format!(
            "Starting emulator {} with {}",
            emulator.description, machine.name
        ));

        let handle =
            thread::spawn(move || run_with_emulator(&machine, system_name, &emulator, rom, &paths));

        match handle.join() {
            Ok(_) => {}
            Err(e) => {
                self.ui_state
                    .add_message(format!("Error starting emulator: {:?}", e));
            }
        }
    }

    fn add_software_list_data_file(&mut self) {
        let dat_file_folder = self.paths.software_lists_data_files_folder.clone();
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            // NOTE: set_directory for Linux seems to be working for GTK only, see set_directory comments
            if let Some(path) = FileDialog::new().set_directory(dat_file_folder).pick_file() {
                sender.send(Some(path)).unwrap();
            } else {
                sender.send(None).unwrap();
            }
        });

        self.file_dialog_receiver = Some(receiver);
    }

    fn scan_available_files(&mut self) {
        self.ui_state.scan_files_dialog_options.show = true;
    }

    fn close_available_files_dialog(&mut self, s_list_id: Option<i32>) {
        self.ui_state.scan_files_dialog_options.show = false;
        if let Some(id) = s_list_id {
            self.ui_state
                .scan_files_dialog_options
                .selected_software_list_id = id;

            let rom_path: PathBuf = PathBuf::from(&self.paths.software_lists_roms_folder);
            let selected_software_list = self
                .ui_state
                .scan_files_dialog_options
                .software_lists
                .iter()
                .find(|s| s.id == id)
                .unwrap()
                .clone();

            let (sender, receiver) = mpsc::channel();
            thread::spawn(move || {
                let mut scanner = SoftwareListFileScanner::new(rom_path);
                let result = scanner.scan_files(&selected_software_list);
                sender.send(Some(result)).unwrap();
            });

            self.software_list_file_scanner_receiver = Some(receiver);
        }
    }

    fn check_software_list_file_scanner_receiver(&mut self) {
        if let Some(receiver) = &self.software_list_file_scanner_receiver {
            if let Ok(result) = receiver.try_recv() {
                if let Some(result) = result {
                    match result {
                        Ok(scan_result) => {
                            self.ui_state.update_matched_files(scan_result);
                        }
                        Err(e) => self
                            .ui_state
                            .add_message(format!("Error scanning files: {}", e.message)),
                    }
                }
                self.software_list_file_scanner_receiver = None;
            }
        }
    }

    fn check_file_dialog_receiver(&mut self) {
        if let Some(receiver) = &self.file_dialog_receiver {
            if let Ok(path) = receiver.try_recv() {
                if let Some(path) = path {
                    self.ui_state.process_from_datafile(path);
                }
                self.file_dialog_receiver = None;
            }
        }
    }
}

impl eframe::App for MameSoftwareListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Add Software Lists data file").clicked() {
                        self.add_software_list_data_file();
                    }
                    if ui
                        .button("Scan available files for a software list")
                        .clicked()
                    {
                        self.scan_available_files();
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
                    self.start_button_clicked();
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
                    |id: Option<i32>| self.close_available_files_dialog(id),
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

            self.check_file_dialog_receiver();
            self.check_software_list_file_scanner_receiver();
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
