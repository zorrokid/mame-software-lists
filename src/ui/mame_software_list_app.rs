use diesel::SqliteConnection;
use eframe::egui;
use crate::{database::software_lists::db_get_software_lists_for_system, models::{Machine, System, SoftwareList}};

use super::systems_combobox::show_systems_combobox;

pub struct MameSoftwareListApp {
    systems: Vec<System>,
    selected_system_id: i32,
    previous_selected_system_id: i32,
    machines: Vec<Machine>,
    connection: Box<SqliteConnection>,
    software_lists: Vec<SoftwareList>,
}

impl MameSoftwareListApp {
    pub fn new(connection: Box<SqliteConnection>, systems: Vec<System>) -> Self {
        Self {
            systems,
            selected_system_id: 0,
            previous_selected_system_id: 0,
            machines: Vec::new(),
            software_lists: Vec::new(),
            connection,
        }
    }

    fn fetch_software_lists_for_system(&mut self, system_id: i32) {
        self.software_lists = db_get_software_lists_for_system(
            self.connection.as_mut(), 
            system_id
        ).unwrap();
    }
}

impl eframe::App for MameSoftwareListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to manage Mame Software Lists");
            
            let mut new_selected_systemid = None;

            show_systems_combobox(
                ui, 
                &self.systems,
                &mut self.selected_system_id, 
                &mut self.previous_selected_system_id, 
                &mut new_selected_systemid,
            );

            if let Some(system_id) = new_selected_systemid {
                self.fetch_software_lists_for_system(system_id);
                self.previous_selected_system_id = system_id;
            }

            if ui.button("Click me").clicked() {
                println!("Selected system: {:?}", self.selected_system_id);
            }
        });
    }
}