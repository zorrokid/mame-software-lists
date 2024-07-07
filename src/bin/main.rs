use eframe::egui;
use mame_software_lists::systems::systems::read_systems;
use mame_software_lists::systems::systems::System;
use mame_software_lists::models::Machine;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let systems = read_systems("configs/systems.json".to_string());
    let app = MyApp::new(systems);
    eframe::run_native(
        "Mame Software Lists", 
        options, 
        Box::new(|_cc| Ok(Box::new(app)))
    )
}

#[derive(Default)]
struct MyApp {
    systems: Vec<System>,
    selected_system_id: String,
    previous_selected_system_id: String,
    machines: Vec<Machine>,
}

impl MyApp {
    fn new(systems: Vec<System>) -> Self {
        Self {
            systems,
            selected_system_id: String::new(),
            previous_selected_system_id: String::new(),
            machines: Vec::new(),
            ..Default::default()
        }
    }

    fn fetch_data_for_system(&mut self, system_id: &String) {
        println!("Fetching data for system: {:?}", system_id);
        self.machines = vec![
            Machine {
                id: 1,
                description: "Description 1".to_string(),
                year: Some(2021),
                name: "Machine 1".to_string(),
                publisher: "Publisher 1".to_string(),
                software_list_id: 1,
            },
        ];
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to manage Mame Software Lists");
            

            let mut new_selected_systemid = None;

            egui::ComboBox::from_label("Systems")
                .selected_text(
                    &self.systems
                        .iter()
                        .find(|s| s.id == self.selected_system_id)
                        .map(|s| s.name.clone())
                        .unwrap_or_default()
                )
                .show_ui(ui, |ui| {
                    for system in self.systems.iter() {
                        if ui.selectable_value(&mut self.selected_system_id, system.id.clone(), system.name.clone()).clicked(){
                            if self.selected_system_id != self.previous_selected_system_id {
                                new_selected_systemid = Some(self.selected_system_id.clone());
                           }
                        }
                    }
                });

            if let Some(system_id) = new_selected_systemid {
                self.fetch_data_for_system(&system_id);
                self.previous_selected_system_id = system_id;
            }

            if ui.button("Click me").clicked() {
                println!("Selected system: {:?}", self.selected_system_id);
            }
        });
    }
}