use mame_software_lists::database::establish_connection;
use mame_software_lists::database::systems::db_get_systems;
use mame_software_lists::ui::mame_software_list_app::MameSoftwareListApp;

fn main() -> Result<(), eframe::Error> {
    let mut connection = Box::new(establish_connection());
    let options = eframe::NativeOptions::default();
    let systems = db_get_systems(connection.as_mut()).unwrap();
    let app = MameSoftwareListApp::new(connection, systems);
    eframe::run_native(
        "Mame Software Lists",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}
