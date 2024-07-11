use crate::{
    configuration::emulators::{get_emulators_by_system_id, Emulator},
    database::{
        machines::db_get_machines_for_software_list,
        software_lists::db_get_software_lists_for_system,
    },
    emulators::emulator_runner::run_with_emulator,
    models::{Machine, SoftwareList, System},
};
use diesel::SqliteConnection;
use eframe::egui;
use std::thread;

use super::{
    emulators_combobox::show_emulators_combobox, machines_list::show_machines_list,
    software_lists_combobox::show_software_lists_combobox, systems_combobox::show_systems_combobox,
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
}

impl MameSoftwareListApp {
    pub fn new(connection: Box<SqliteConnection>, systems: Vec<System>) -> Self {
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

    fn fetch_emulators_for_system(&mut self, system_name: String) {
        self.emulators = get_emulators_by_system_id(system_name).unwrap()
    }

    fn start_button_clicked(&self) {
        println!("Selected system: {:?}", self.selected_system_id);
        if self.selected_machine_id != 0 && self.selected_emulator_id != "" {
            println!("Selected machine: {:?}", self.selected_machine_id);
            println!(
                "Selected software list: {:?}",
                self.selected_software_list_id
            );
            println!("Selected emulator: {:?}", self.selected_emulator_id);

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
                        println!("Error starting emulator {}", e);
                    }
                }
            });
            handle.join().unwrap();
        }
    }
}

impl eframe::App for MameSoftwareListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to start software from Mame Software Lists");

            let mut new_selected_systemid = None;
            let mut new_selected_software_list_id = None;
            let mut new_selected_machine_id = None;

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

            show_machines_list(
                ui,
                &self.machines,
                &mut self.selected_machine_id,
                &mut self.previous_selected_machine_id,
                &mut new_selected_machine_id,
            );

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
                self.previous_selected_machine_id = machine_id;
            }
        });
    }
}
