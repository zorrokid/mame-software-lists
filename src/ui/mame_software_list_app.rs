use crate::{
    configuration::{
        emulators::{get_emulators_by_system_id, Emulator},
        paths::Paths,
    },
    data_access::data_access_provider::{DataAccessProvider, DataAccessTrait},
    emulators::emulator_runner::run_with_emulator,
    models::{Machine, SoftwareList, System},
    software_lists::{
        process::process_from_datafile,
        software_list_file_scanner::{
            SoftwareListFileScanner, SoftwareListScannerError, SoftwareListScannerResult,
        },
    },
};
use eframe::egui;
use rfd::FileDialog;
use std::{path::PathBuf, sync::mpsc, thread};

use super::{
    emulators_combobox::show_emulators_combobox,
    machines_list::show_machines_list,
    message_dialog::{show_message_dialog, MessageDialogOptions},
    rom_selection_options::RomSelectionOptions,
    roms_list::show_roms_list,
    scan_files_dialog::{show_scan_files_dialog, ScanFilesDialogOptions},
    software_lists_combobox::show_software_lists_combobox,
    systems_combobox::show_systems_combobox,
};

#[derive(Clone)]
pub struct SystemSelectionOptions {
    pub selected_system_id: i32,
    pub previous_selected_system_id: i32,
    pub systems: Vec<System>,
}

pub struct MameSoftwareListApp {
    selected_software_list_id: i32,
    previous_selected_software_list_id: i32,
    selected_machine_id: i32,
    previous_selected_machine_id: i32,
    machines: Vec<Machine>,
    software_lists_for_selected_system: Vec<SoftwareList>,
    emulators: Vec<Emulator>,
    selected_emulator_id: String,
    paths: Paths,
    file_dialog_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
    error_messages: Vec<String>,
    data_access: DataAccessProvider,
    software_list_file_scanner_receiver:
        Option<mpsc::Receiver<Option<Result<SoftwareListScannerResult, SoftwareListScannerError>>>>,
    message_dialog_options: MessageDialogOptions,
    scan_files_dialog_options: ScanFilesDialogOptions,
    rom_selection_options: RomSelectionOptions,
    system_selection_options: SystemSelectionOptions,
}

impl MameSoftwareListApp {
    pub fn new(paths: Paths) -> Self {
        let mut data_access = DataAccessProvider::new();
        let systems = data_access
            .get_systems()
            .map_err(|e| println!("Failed getting systems: {}", e.message))
            .unwrap();

        let software_lists = data_access
            .get_software_lists()
            .map_err(|e| println!("Failed getting software lists: {}", e.message))
            .unwrap();

        Self {
            selected_software_list_id: 0,
            previous_selected_software_list_id: 0,
            selected_machine_id: 0,
            previous_selected_machine_id: 0,
            machines: Vec::new(),
            software_lists_for_selected_system: Vec::new(),
            emulators: Vec::new(),
            selected_emulator_id: String::new(),
            paths,
            file_dialog_receiver: None,
            error_messages: Vec::new(),
            data_access,
            software_list_file_scanner_receiver: None,
            message_dialog_options: MessageDialogOptions {
                show: false,
                message: String::new(),
            },
            scan_files_dialog_options: ScanFilesDialogOptions {
                show: false,
                software_lists,
                selected_software_list_id: 0,
            },
            rom_selection_options: RomSelectionOptions::new(0, 0, None, Vec::new()),
            system_selection_options: SystemSelectionOptions {
                selected_system_id: 0,
                previous_selected_system_id: 0,
                systems,
            },
        }
    }

    fn fetch_software_lists_for_system(&mut self, system_id: i32) {
        self.software_lists_for_selected_system =
            match self.data_access.get_software_lists_for_system(system_id) {
                Ok(s_lists) => s_lists,
                Err(e) => {
                    self.error_messages.push(e.message);
                    Vec::new()
                }
            }
    }

    fn fetch_machines_for_software_list(&mut self, s_list_id: i32) {
        self.machines = match self.data_access.get_machines_for_software_list(s_list_id) {
            Ok(machines) => machines,
            Err(e) => {
                self.error_messages.push(e.message);
                Vec::new()
            }
        }
    }

    fn fetch_roms_for_machine(&mut self, machine_id: i32) {
        let machine = self.machines.iter().find(|m| m.id == machine_id).unwrap();
        self.rom_selection_options
            .set_roms(match self.data_access.get_roms_for_machine(machine) {
                Ok(roms) => roms,
                Err(e) => {
                    self.error_messages.push(e.message);
                    Vec::new()
                }
            })
    }

    fn fetch_emulators_for_system(&mut self, system_name: String) {
        self.emulators = match get_emulators_by_system_id(system_name) {
            Ok(emulators) => emulators,
            Err(e) => {
                self.error_messages.push(e.message.clone());
                Vec::new()
            }
        }
    }

    fn fetch_systems(&mut self) {
        self.system_selection_options.systems = match self.data_access.get_systems() {
            Ok(systems) => systems,
            Err(e) => {
                self.error_messages.push(e.message);
                Vec::new()
            }
        }
    }

    fn get_selected_system(&self) -> Option<&System> {
        self.system_selection_options
            .systems
            .iter()
            .find(|s| s.id == self.system_selection_options.selected_system_id)
    }

    fn get_selected_machine(&self) -> Option<&Machine> {
        self.machines
            .iter()
            .find(|m| m.id == self.selected_machine_id)
    }

    fn start_button_clicked(&mut self) {
        if self.selected_machine_id != 0 && self.selected_emulator_id != "" {
            let system_name = self.get_selected_system().unwrap().name.clone();
            let machine = self.get_selected_machine().unwrap().clone();
            let emulator_id = self.selected_emulator_id.clone();
            let rom = self
                .rom_selection_options
                .get_selected_rom()
                .cloned()
                .map(|r| r.clone());

            let handle =
                thread::spawn(move || run_with_emulator(&machine, system_name, emulator_id, rom));

            match handle.join() {
                Ok(_) => {}
                Err(e) => {
                    self.error_messages
                        .push(format!("Error starting emulator: {:?}", e));
                }
            }
        }
    }

    fn add_software_list_data_file(&mut self) {
        let dat_file_folder = self.paths.software_lists_data_files_folder.clone();
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            if let Some(path) = FileDialog::new().set_directory(dat_file_folder).pick_file() {
                sender.send(Some(path)).unwrap();
            } else {
                sender.send(None).unwrap();
            }
        });

        self.file_dialog_receiver = Some(receiver);
    }

    fn scan_available_files(&mut self) {
        self.scan_files_dialog_options.show = true;
    }

    fn close_available_files_dialog(&mut self, s_list_id: Option<i32>) {
        self.scan_files_dialog_options.show = false;
        if let Some(id) = s_list_id {
            self.scan_files_dialog_options.selected_software_list_id = id;

            let rom_path: PathBuf =
                PathBuf::from(self.paths.software_lists_roms_folder.clone()).clone();
            let selected_software_list = self
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

    fn updated_matched_files(&mut self, result: SoftwareListScannerResult) {
        let matching_files_count = self
            .data_access
            .set_matched_roms(&result.software_list, &result.scan_result.found_checksums)
            .map_err(|e| {
                self.error_messages.push(e.message);
            })
            .unwrap();

        self.message_dialog_options = MessageDialogOptions {
            show: true,
            message: format!("Matching files count: {:?}", matching_files_count),
        };
    }

    fn check_software_list_file_scanner_receiver(&mut self) {
        if let Some(receiver) = &self.software_list_file_scanner_receiver {
            if let Ok(result) = receiver.try_recv() {
                if let Some(result) = result {
                    match result {
                        Ok(scan_result) => {
                            self.updated_matched_files(scan_result);
                        }
                        Err(e) => {
                            self.error_messages.push(e.message);
                        }
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
                    println!("Selected file: {:?} ... start processing", path);
                    match process_from_datafile(
                        &mut self.data_access,
                        path.to_string_lossy().into_owned(),
                    ) {
                        Ok(_) => {
                            println!("Software list processed");
                        }
                        Err(e) => {
                            self.error_messages
                                .push(format!("Error processing software list: {}", e.message));
                        }
                    }
                }
                self.file_dialog_receiver = None;
                self.fetch_systems();
            }
        }
    }

    fn on_system_id_changed(&mut self, system_id: i32) {
        self.fetch_software_lists_for_system(system_id);
        let system_name = self
            .system_selection_options
            .systems
            .iter()
            .find(|s| s.id == system_id)
            .unwrap()
            .name
            .clone();
        self.fetch_emulators_for_system(system_name);
        self.system_selection_options.previous_selected_system_id = system_id;
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
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to start software from Mame Software Lists");

            let mut new_selected_software_list_id = None;
            let mut new_selected_machine_id = None;

            let system_selection_options = self.system_selection_options.clone();
            ui.horizontal(|ui| {
                show_systems_combobox(ui, system_selection_options, &mut |id| {
                    self.on_system_id_changed(id)
                });

                show_software_lists_combobox(
                    ui,
                    &self.software_lists_for_selected_system,
                    &mut self.selected_software_list_id,
                    &mut self.previous_selected_software_list_id,
                    &mut new_selected_software_list_id,
                );

                show_emulators_combobox(ui, &self.emulators, &mut self.selected_emulator_id);

                if ui.button("Start").clicked() {
                    self.start_button_clicked();
                }
            });

            ui.add_sized(ui.available_size(), |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    show_machines_list(
                        ui,
                        &self.machines,
                        &mut self.selected_machine_id,
                        &mut self.previous_selected_machine_id,
                        &mut new_selected_machine_id,
                    );
                    show_roms_list(ui, &mut self.rom_selection_options);
                })
                .response
            });

            if let Some(s_list_id) = new_selected_software_list_id {
                self.fetch_machines_for_software_list(s_list_id);
                self.previous_selected_software_list_id = s_list_id;
            }

            if let Some(machine_id) = new_selected_machine_id {
                self.fetch_roms_for_machine(machine_id);
                self.previous_selected_machine_id = machine_id;
            }

            if self.scan_files_dialog_options.show {
                let cloned_software_lists = self.scan_files_dialog_options.software_lists.clone();
                let selected_software_list_id =
                    self.scan_files_dialog_options.selected_software_list_id;
                show_scan_files_dialog(
                    ctx,
                    |id: Option<i32>| self.close_available_files_dialog(id),
                    &cloned_software_lists,
                    selected_software_list_id,
                );
            }

            if self.message_dialog_options.show {
                show_message_dialog(ctx, &self.message_dialog_options.message, &mut || {
                    self.message_dialog_options.show = false;
                })
            }

            self.check_file_dialog_receiver();
            self.check_software_list_file_scanner_receiver();
        });

        egui::TopBottomPanel::bottom("error_console").show(ctx, |ui| {
            ui.heading("Error Console");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for error in &self.error_messages {
                    ui.label(error);
                }
            });
        });
    }
}
