use mame_software_lists::ui::mame_software_list_app::MameSoftwareListApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    let app = MameSoftwareListApp::new();
    eframe::run_native(
        "Mame Software Lists",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
