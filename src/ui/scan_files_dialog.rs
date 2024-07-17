use eframe::egui;

pub fn show_scan_files_dialog<F>(ctx: &egui::Context, mut close_dialog: F)
where
    F: FnMut() -> (),
{
    egui::Window::new("Scan Files").show(ctx, |ui| {
        ui.label("Scan Files");
        if ui.button("Scan").clicked() {
            println!("Scan button clicked");
        }
        if ui.button("Cancel").clicked() {
            println!("Cancel button clicked");
            close_dialog();
        }
    });
    println!("show_scan_files_dialog");
}
