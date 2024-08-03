use eframe::egui;

use crate::models::SoftwareList;

pub struct ScanFilesDialogOptions {
    pub software_lists: Vec<SoftwareList>,
    pub selected_software_list_id: i32,
    pub show: bool,
}
pub struct ScanFilesDialog<'a> {
    close_dialog: &'a mut dyn FnMut(Option<i32>),
    software_lists: &'a Vec<SoftwareList>,
    selected_software_list_id: i32,
}

impl<'a> ScanFilesDialog<'a> {
    pub fn new(
        close_dialog: &'a mut dyn FnMut(Option<i32>),
        software_lists: &'a Vec<SoftwareList>,
        selected_software_list_id: i32,
    ) -> Self {
        Self {
            close_dialog,
            software_lists,
            selected_software_list_id,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Scan Files").show(ctx, |ui| {
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
                        (self.close_dialog)(Some(software_list.id));
                    }
                    ui.end_row();
                }
            });
            if ui.button("Cancel").clicked() {
                (self.close_dialog)(None);
            }
        });
    }
}
