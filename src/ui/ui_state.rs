use super::{
    machines_list::MachineSelectionOptions, message_dialog::MessageDialogOptions,
    selection_options::SelectionOptions,
    software_list_selection_dialog::SoftwareListSelectionDialogOptions,
};
use crate::configuration::{
    emulators::{get_emulators_by_system_id, Emulator},
    paths::{read_paths, Paths},
};
use crate::data_access::data_access_provider::{DataAccessProvider, DataAccessTrait};
use crate::emulators::emulator_runner::run_with_emulator;
use crate::models::{Machine, Rom, SoftwareList, System};
use crate::software_lists::{
    process::process_from_datafile,
    software_list_file_scanner::{
        SoftwareListFileScanner, SoftwareListScannerError, SoftwareListScannerResult,
    },
};
use rfd::FileDialog;
use std::{path::PathBuf, sync::mpsc, thread};

pub struct UiState {
    data_access: DataAccessProvider,
    pub system_selection_options: SelectionOptions<System>,
    pub software_list_selection_options: SelectionOptions<SoftwareList>,
    pub machine_selection_options: MachineSelectionOptions,
    pub emulator_selection_options: SelectionOptions<Emulator>,
    pub rom_selection_options: SelectionOptions<Rom>,
    pub message_dialog_options: MessageDialogOptions,
    pub console_messages: Vec<String>,
    pub software_list_selection_dialog_options: SoftwareListSelectionDialogOptions,
    paths: Paths,
    file_dialog_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
    software_list_file_scanner_receiver:
        Option<mpsc::Receiver<Option<Result<SoftwareListScannerResult, SoftwareListScannerError>>>>,
}

impl UiState {
    pub fn new() -> Self {
        let mut data_access = DataAccessProvider::new();

        let systems = data_access
            .get_systems()
            .map_err(|e| println!("Failed getting systems: {}", e.message))
            .unwrap();

        let paths = read_paths();

        Self {
            data_access,
            system_selection_options: SelectionOptions::<System> {
                selected: None,
                items: systems,
            },
            software_list_selection_options: SelectionOptions::<SoftwareList>::default(),
            machine_selection_options: MachineSelectionOptions::default(),
            emulator_selection_options: SelectionOptions::<Emulator>::default(),
            rom_selection_options: SelectionOptions::<Rom>::default(),
            message_dialog_options: MessageDialogOptions {
                show: false,
                message: String::new(),
            },
            software_list_selection_dialog_options: SoftwareListSelectionDialogOptions {
                show: false,
            },
            console_messages: Vec::new(),
            paths,
            file_dialog_receiver: None,
            software_list_file_scanner_receiver: None,
        }
    }

    pub fn close_message_dialog(&mut self) {
        self.message_dialog_options.show = false;
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
            self.fetch_software_lists_for_system(&system);
            self.fetch_emulators_for_system(&system);
        }
        self.system_selection_options.selected = system;
    }

    pub fn on_software_list_selection_changed(&mut self, software_list: Option<SoftwareList>) {
        if let Some(software_list) = software_list.clone() {
            self.fetch_machines_for_software_list(&software_list);
        }
        self.software_list_selection_options.selected = software_list;
    }

    pub fn on_machine_selection_changed(&mut self, machine: Option<Machine>) {
        if let Some(machine) = machine.clone() {
            self.fetch_roms_for_machine(&machine);
        }
        self.machine_selection_options.selected = machine;
    }

    pub fn on_emulator_changed(&mut self, emulator: Option<Emulator>) {
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

    pub fn add_software_list_data_file(&mut self) {
        let dat_file_folder = self.paths.software_lists_data_files_folder.clone();
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            // NOTE: set_directory for Linux seems to be working for GTK only, see set_directory comments
            if let Some(path) = FileDialog::new().set_directory(dat_file_folder).pick_file() {
                sender.send(Some(path))
            } else {
                sender.send(None)
            }
        });

        self.file_dialog_receiver = Some(receiver);
    }

    pub fn scan_available_files(&mut self) {
        self.software_list_selection_dialog_options.show = true;
    }

    pub fn close_software_list_selection_dialog(&mut self, software_list: Option<&SoftwareList>) {
        self.software_list_selection_dialog_options.show = false;
        if let Some(s_list) = software_list {
            let rom_path: PathBuf = PathBuf::from(&self.paths.software_lists_roms_folder);
            let (sender, receiver) = mpsc::channel();
            let software_list_cloned = s_list.clone();
            thread::spawn(move || {
                let mut scanner = SoftwareListFileScanner::new(rom_path);
                let result = scanner.scan_files(&software_list_cloned);
                sender.send(Some(result))
            });

            self.software_list_file_scanner_receiver = Some(receiver);
        }
    }

    pub fn on_update(&mut self) {
        self.check_software_list_file_scanner_receiver();
        self.check_file_dialog_receiver();
    }

    pub fn get_all_software_lists(&mut self) -> Vec<SoftwareList> {
        let result = self.data_access.get_software_lists();
        match result {
            Ok(s_lists) => s_lists,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }

    // private

    fn check_software_list_file_scanner_receiver(&mut self) {
        if let Some(receiver) = &self.software_list_file_scanner_receiver {
            if let Ok(result) = receiver.try_recv() {
                if let Some(result) = result {
                    match result {
                        Ok(scan_result) => {
                            self.update_matched_files(scan_result);
                        }
                        Err(e) => self.add_message(format!("Error scanning files: {}", e.message)),
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
                    self.process_from_datafile(path);
                }
                self.file_dialog_receiver = None;
            }
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

    fn fetch_software_lists_for_system(&mut self, system: &System) {
        self.software_list_selection_options.items =
            match self.data_access.get_software_lists_for_system(system.id) {
                Ok(s_lists) => s_lists,
                Err(e) => {
                    self.add_message(e.message);
                    Vec::new()
                }
            }
    }

    fn fetch_emulators_for_system(&mut self, system: &System) {
        self.emulator_selection_options.items = match get_emulators_by_system_id(&system.name) {
            Ok(emulators) => emulators,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }
    fn fetch_machines_for_software_list(&mut self, software_list: &SoftwareList) {
        self.machine_selection_options.items = match self
            .data_access
            .get_machines_for_software_list(software_list.id)
        {
            Ok(machines) => machines,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }

    fn fetch_roms_for_machine(&mut self, machine: &Machine) {
        self.rom_selection_options.items = match self.data_access.get_roms_for_machine(machine) {
            Ok(roms) => roms,
            Err(e) => {
                self.add_message(e.message);
                Vec::new()
            }
        }
    }
}
