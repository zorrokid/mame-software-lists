use eframe::egui;

use crate::models::SoftwareList;

pub struct SoftwareListSelectionDialogOptions {
    pub show: bool,
}
pub struct SoftwareListSelectionDialog<'a> {
    close_dialog: &'a mut dyn FnMut(Option<&SoftwareList>),
    software_lists: &'a Vec<SoftwareList>,
}

impl<'a> SoftwareListSelectionDialog<'a> {
    pub fn new(
        close_dialog: &'a mut dyn FnMut(Option<&SoftwareList>),
        software_lists: &'a Vec<SoftwareList>,
    ) -> Self {
        Self {
            close_dialog,
            software_lists,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut selected: Option<&SoftwareList> = None;
        egui::Window::new("Software lists").show(ctx, |ui| {
            egui::Grid::new("software_lists").show(ui, |ui| {
                ui.label("Software List");
                ui.end_row();

                for software_list in self.software_lists.iter() {
                    if ui
                        .selectable_value(
                            &mut selected,
                            Some(software_list),
                            software_list.name.clone(),
                        )
                        .clicked()
                    {
                        (self.close_dialog)(selected);
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
