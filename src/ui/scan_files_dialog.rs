use eframe::egui;

use crate::models::SoftwareList;

pub struct ScanFilesDialog<'a> {
    close_dialog: &'a mut dyn FnMut(),
    ctx: &'a egui::Context,
    software_lists: &'a Vec<SoftwareList>,
    selected_software_list_id: i32,
}

impl<'a> ScanFilesDialog<'a> {
    pub fn new(
        close_dialog: &'a mut dyn FnMut(),
        ctx: &'a egui::Context,
        software_lists: &'a Vec<SoftwareList>,
    ) -> Self {
        Self {
            close_dialog,
            ctx,
            software_lists,
            selected_software_list_id: 0,
        }
    }

    fn scan_files_for_software_list(&self, software_list_id: i32) {
        println!("scan_files_for_software_list");
        // TODO
    }

    pub fn show(&mut self) {
        egui::Window::new("Scan Files").show(self.ctx, |ui| {
            egui::Grid::new("software_lists").show(ui, |ui| {
                ui.label("Software List");
                ui.end_row();

                for software_list in self.software_lists.iter() {
                    if ui
                        .selectable_value(
                            &mut self.selected_software_list_id,
                            software_list.id.clone(),
                            software_list.name.clone(),
                        )
                        .clicked()
                    {
                        self.scan_files_for_software_list(software_list.id);
                    }
                    ui.end_row();
                }
            });
            ui.label("Scan Files");
            if ui.button("Scan").clicked() {
                println!("Scan button clicked");
            }
            if ui.button("Cancel").clicked() {
                println!("Cancel button clicked");
                (self.close_dialog)();
            }
        });
        println!("show_scan_files_dialog");
    }
}

pub fn show_scan_files_dialog<F>(
    ctx: &egui::Context,
    mut close_dialog: F,
    software_lists: &Vec<SoftwareList>,
) where
    F: FnMut() -> (),
{
    ScanFilesDialog::new(&mut close_dialog, ctx, software_lists).show();
}
