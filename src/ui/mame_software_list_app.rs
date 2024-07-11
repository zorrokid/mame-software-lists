use crate::{
    configuration::{emulators::{get_emulators_by_system_id, Emulator}, paths::Paths},
    database::{
        machines::db_get_machines_for_software_list,
        software_lists::db_get_software_lists_for_system,
        systems::db_get_systems,
        roms::db_get_roms,
    },
    emulators::emulator_runner::run_with_emulator,
    models::{Machine, SoftwareList, System, Rom}, software_lists::process::process_from_datafile,
};
use diesel::SqliteConnection;
use eframe::egui;
use rfd::FileDialog;
use std::{path::PathBuf, sync::mpsc, thread};

use super::{
    emulators_combobox::show_emulators_combobox, machines_list::show_machines_list,
    software_lists_combobox::show_software_lists_combobox, systems_combobox::show_systems_combobox,
    roms_list::show_roms_list,
};

pub struct MameSoftwareListApp {
    systems: Vec<System>,
    selected_system_id: i32,
    previous_selected_system_id: i32,
    selected_software_list_id: i32,
    previous_selected_software_list_id: i32,
    selected_machine_id: i32,
    previous_selected_machine_id: i32,
    machines: Vec<Machine>,
    connection: Box<SqliteConnection>,
    software_lists: Vec<SoftwareList>,
    emulators: Vec<Emulator>,
    selected_emulator_id: String,
    paths: Paths,
    file_dialog_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,
    error_messages: Vec<String>,
    roms: Vec<Rom>,
    selected_rom_id: i32,
    previous_selected_rom_id: i32,
}

impl MameSoftwareListApp {
    pub fn new(connection: Box<SqliteConnection>, systems: Vec<System>, paths: Paths) -> Self {
        Self {
            systems,
            selected_system_id: 0,
            previous_selected_system_id: 0,
            selected_software_list_id: 0,
            previous_selected_software_list_id: 0,
            selected_machine_id: 0,
            previous_selected_machine_id: 0,
            machines: Vec::new(),
            software_lists: Vec::new(),
            connection,
            emulators: Vec::new(),
            selected_emulator_id: String::new(),
            paths,
            file_dialog_receiver: None,
            error_messages: Vec::new(),
            roms: Vec::new(),
            selected_rom_id: 0,
            previous_selected_rom_id: 0,
        }
    }

    fn fetch_software_lists_for_system(&mut self, system_id: i32) {
        self.software_lists =
            db_get_software_lists_for_system(self.connection.as_mut(), system_id).unwrap();
    }

    fn fetch_machines_for_software_list(&mut self, s_list_id: i32) {
        self.machines =
            db_get_machines_for_software_list(self.connection.as_mut(), s_list_id).unwrap();
    }

    fn fetch_roms_for_machine(&mut self, machine_id: i32) {
        let machine = self
            .machines
            .iter()
            .find(|m| m.id == machine_id)
            .unwrap();
        self.roms = db_get_roms(self.connection.as_mut(), machine).unwrap();
    }

    fn fetch_emulators_for_system(&mut self, system_name: String) {
        self.emulators = match get_emulators_by_system_id(system_name) {
            Ok(emulators) => emulators,
            Err(e) => {
                self.error_messages.push(format!("Error getting emulators: {}", e));
                Vec::new()
            }
        }
    }

    fn fetch_systems(&mut self) {   
        self.systems = match db_get_systems(self.connection.as_mut()) {
            Ok(systems) => systems,
            Err(e) => {
                self.error_messages.push(format!("Error getting systems: {}", e));
                Vec::new()
            }
        }
    }

    fn start_button_clicked(&self) {
        if self.selected_machine_id != 0 && self.selected_emulator_id != "" {
        
            // Clone the values to pass them to the thread closure
            let system_name = self
                .systems
                .iter()
                .find(|s| s.id == self.selected_system_id)
                .unwrap()
                .name
                .clone();
            let machine = self
                .machines
                .iter()
                .find(|m| m.id == self.selected_machine_id)
                .unwrap();
            // TODO: why is this now working?
            //let machine_clone = machine.clone();
            let machine_clone = Machine {
                id: self.selected_machine_id,
                name: machine.name.clone(),
                description: machine.description.clone(),
                year: machine.year.clone(),
                publisher: machine.publisher.clone(),
                software_list_id: self.selected_software_list_id,
            };
            let emulator_id = self.selected_emulator_id.clone();
            // start run_with_emulator in a new thread
            let handle = thread::spawn(move || {
                match run_with_emulator(&machine_clone, system_name, emulator_id) {
                    Ok(_) => {
                        println!("Emulator started successfully");
                    }
                    Err(e) => {
                        // TODO: show error in UI
                        println!("Error starting emulator {}", e);
                    }
                }
            });
            handle.join().unwrap();
        }
    }

    fn add_software_list_data_file(&mut self) {
        let dat_file_folder = self.paths.software_lists_data_files_folder.clone();
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            if let Some(path) = FileDialog::new()
                .set_directory(dat_file_folder)
                .pick_file()
            {
                sender.send(Some(path)).unwrap();
            } else {
                sender.send(None).unwrap();
            }
        });

        self.file_dialog_receiver = Some(receiver);

    }

    fn check_file_dialog_receiver(&mut self) {
        if let Some(receiver) = &self.file_dialog_receiver {
            if let Ok(path) = receiver.try_recv() {
                if let Some(path) = path {
                    println!("Selected file: {:?} ... start processing", path);
                    process_from_datafile(self.connection.as_mut(), path.to_string_lossy().into_owned());
                    println!("Processing finished");
                }
                self.file_dialog_receiver = None;
                self.fetch_systems();
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
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
           })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to start software from Mame Software Lists");

            let mut new_selected_systemid = None;
            let mut new_selected_software_list_id = None;
            let mut new_selected_machine_id = None;
            let mut new_selected_rom_id = None;

            ui.horizontal(|ui| {
                show_systems_combobox(
                    ui,
                    &self.systems,
                    &mut self.selected_system_id,
                    &mut self.previous_selected_system_id,
                    &mut new_selected_systemid,
                );

                show_software_lists_combobox(
                    ui,
                    &self.software_lists,
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
                    show_roms_list(
                        ui,
                        &self.roms,
                        &mut self.selected_rom_id,
                        &mut self.previous_selected_rom_id,
                        &mut new_selected_rom_id,
                    );
                }).response
            });

            if let Some(system_id) = new_selected_systemid {
                self.fetch_software_lists_for_system(system_id);
                let system_name = self
                    .systems
                    .iter()
                    .find(|s| s.id == system_id)
                    .unwrap()
                    .name
                    .clone();
                self.fetch_emulators_for_system(system_name);
                self.previous_selected_system_id = system_id;
            }

            if let Some(s_list_id) = new_selected_software_list_id {
                self.fetch_machines_for_software_list(s_list_id);
                self.previous_selected_software_list_id = s_list_id;
            }

            if let Some(machine_id) = new_selected_machine_id {
                self.fetch_roms_for_machine(machine_id);
                self.previous_selected_machine_id = machine_id;
            }

            if let Some(rom_id) = new_selected_rom_id {
                self.previous_selected_rom_id = rom_id;
            }

            self.check_file_dialog_receiver();
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
