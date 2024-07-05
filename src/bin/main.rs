use eframe::egui;
use mame_software_lists::systems::systems::read_systems;
use mame_software_lists::systems::systems::System;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let systems = read_systems("configs/systems.json".to_string());
    eframe::run_native(
        "Mame Software Lists", 
        options, 
        Box::new(|_cc| Ok(Box::new(MyApp::new(systems))))
    )
}

#[derive(Default)]
struct MyApp {
    systems: Vec<System>,
    selected_system_id: String,
}

impl MyApp {
    fn new(systems: Vec<System>) -> Self {
        Self {
            systems,
            ..Default::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mame Software Lists");
            ui.label("This is a simple app to manage Mame Software Lists");
            

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
                        ui.selectable_value(&mut self.selected_system_id, system.id.clone(), system.name.clone());
                    }
                });

            if ui.button("Click me").clicked() {
                println!("Selected system: {:?}", self.selected_system_id);
            }
        });
    }
}