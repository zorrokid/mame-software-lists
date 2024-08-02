use crate::configuration::emulators::{get_emulators_by_system_id, Emulator};
use crate::configuration::paths::{read_paths, Paths};
use crate::data_access::data_access_provider::{DataAccessProvider, DataAccessTrait};
use crate::emulators::emulator_runner::run_with_emulator;
use crate::models::System;
use crate::models::{Machine, Rom, SoftwareList};
use crate::software_lists::{
    process::process_from_datafile, software_list_file_scanner::SoftwareListScannerResult,
};
use crate::ui::combobox::{
    emulators_combobox::EmulatorSelectionOptions,
    software_lists_combobox::SoftwareListSelectionOptions,
    systems_combobox::SystemSelectionOptions,
};
use crate::ui::machines_list::MachineSelectionOptions;
use crate::ui::message_dialog::MessageDialogOptions;
use crate::ui::roms_list::RomSelectionOptions;
use crate::ui::scan_files_dialog::ScanFilesDialogOptions;
use std::path::PathBuf;
use std::thread;

pub struct UiState {
    data_access: DataAccessProvider,
    pub system_selection_options: SystemSelectionOptions,
    pub software_list_selection_options: SoftwareListSelectionOptions,
    pub machine_selection_options: MachineSelectionOptions,
    pub emulator_selection_options: EmulatorSelectionOptions,
    pub rom_selection_options: RomSelectionOptions,
    pub message_dialog_options: MessageDialogOptions,
    pub console_messages: Vec<String>,
    pub scan_files_dialog_options: ScanFilesDialogOptions,
    paths: Paths,
}

impl UiState {
    pub fn new() -> Self {
        let mut data_access = DataAccessProvider::new();

        let systems = data_access
            .get_systems()
            .map_err(|e| println!("Failed getting systems: {}", e.message))
            .unwrap();

        let software_lists = data_access
            .get_software_lists()
            .map_err(|e| println!("Failed getting software lists: {}", e.message))
            .unwrap();

        let paths = read_paths();

        Self {
            data_access,
            system_selection_options: SystemSelectionOptions {
                selected: None,
                items: systems,
            },
            software_list_selection_options: SoftwareListSelectionOptions::default(),
            machine_selection_options: MachineSelectionOptions::default(),
            emulator_selection_options: EmulatorSelectionOptions::default(),
            rom_selection_options: RomSelectionOptions::default(),
            message_dialog_options: MessageDialogOptions {
                show: false,
                message: String::new(),
            },
            scan_files_dialog_options: ScanFilesDialogOptions {
                show: false,
                software_lists,
                selected_software_list_id: 0,
            },
            console_messages: Vec::new(),
            paths,
        }
    }

    fn fetch_systems(&mut self) {
        self.system_selection_options.items = match self.data_access.get_systems() {
            Ok(systems) => systems,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }

    fn fetch_software_lists_for_system(&mut self, system_id: i32) {
        self.software_list_selection_options.items =
            match self.data_access.get_software_lists_for_system(system_id) {
                Ok(s_lists) => s_lists,
                Err(e) => {
                    self.add_message(e.message);
                    Vec::new()
                }
            }
    }

    fn fetch_emulators_for_system(&mut self, system_name: String) {
        self.emulator_selection_options.items = match get_emulators_by_system_id(system_name) {
            Ok(emulators) => emulators,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }
    fn fetch_machines_for_software_list(&mut self, s_list_id: i32) {
        self.machine_selection_options.items =
            match self.data_access.get_machines_for_software_list(s_list_id) {
                Ok(machines) => machines,
                Err(e) => {
                    self.add_message(e.message);
                    Vec::new()
                }
            }
    }

    fn fetch_roms_for_machine(&mut self, machine_id: i32) {
        let machine = self
            .machine_selection_options
            .items
            .iter()
            .find(|m| m.id == machine_id)
            .unwrap();
        self.rom_selection_options.items = match self.data_access.get_roms_for_machine(machine) {
            Ok(roms) => roms,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }

    pub fn update_matched_files(&mut self, result: SoftwareListScannerResult) {
        let matching_files_count = self
            .data_access
            .set_matched_roms(&result.software_list, &result.scan_result.found_checksums)
            .map_err(|e| {
                self.add_message(e.message);
            })
            .unwrap();

        self.add_message(format!("Matching files count: {:?}", matching_files_count));
    }

    pub fn process_from_datafile(&mut self, path: PathBuf) {
        match process_from_datafile(&mut self.data_access, path.to_string_lossy().into_owned()) {
            Ok(_) => {
                self.fetch_systems();
                self.add_message("Software list processed".to_string());
            }
            Err(e) => self.add_message(format!("Error processing software list: {}", e.message)),
        }
    }

    pub fn add_message(&mut self, message: String) {
        self.message_dialog_options = MessageDialogOptions {
            show: true,
            message: message.clone(),
        };
        self.console_messages.push(message);
    }

    pub fn add_console_message(&mut self, message: String) {
        self.console_messages.push(message);
    }

    pub fn on_system_changed(&mut self, system: Option<System>) {
        if let Some(system) = system.clone() {
            self.fetch_software_lists_for_system(system.id);
            self.fetch_emulators_for_system(system.name);
        }
        self.system_selection_options.selected = system;
    }

    pub fn on_software_list_selection_changed(&mut self, software_list: Option<SoftwareList>) {
        if let Some(software_list) = software_list.clone() {
            self.fetch_machines_for_software_list(software_list.id);
        }
        self.software_list_selection_options.selected = software_list;
    }

    pub fn on_machine_selection_changed(&mut self, machine: Option<Machine>) {
        if let Some(machine) = machine.clone() {
            self.fetch_roms_for_machine(machine.id);
        }
        self.machine_selection_options.selected = machine;
    }

    pub fn on_emulator_id_changed(&mut self, emulator: Option<Emulator>) {
        self.emulator_selection_options.selected = emulator;
    }

    pub fn on_rom_selected(&mut self, selected_rom: Option<Rom>) {
        self.rom_selection_options.selected = selected_rom;
    }

    pub fn start_button_clicked(&mut self) {
        if self.system_selection_options.selected.is_none() {
            self.add_message("Please select a system".to_string());
            return;
        }
        if self.machine_selection_options.selected.is_none() {
            self.add_message("Please select a machine".to_string());
            return;
        }
        if self.emulator_selection_options.selected.is_none() {
            self.add_message("Please select an emulator".to_string());
            return;
        }
        let system_name = self.system_selection_options.selected.clone().unwrap().name;
        let machine = self.machine_selection_options.selected.clone().unwrap();
        let emulator = self.emulator_selection_options.selected.clone().unwrap();
        let rom = self.rom_selection_options.selected.clone();
        let paths = self.paths.clone();

        self.add_console_message(format!(
            "Starting emulator {} with {}",
            emulator.description, machine.name
        ));

        let handle =
            thread::spawn(move || run_with_emulator(&machine, system_name, &emulator, rom, &paths));

        match handle.join() {
            Ok(_) => {}
            Err(e) => {
                self.add_message(format!("Error starting emulator: {:?}", e));
            }
        }
    }
}
