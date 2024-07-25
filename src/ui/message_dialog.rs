pub struct MessageDialogOptions {
    pub message: String,
    pub show: bool,
}

pub struct MessageDialog<'a> {
    ctx: &'a egui::Context,
    message: &'a String,
    close_dialog: &'a mut dyn FnMut(),
}

impl<'a> MessageDialog<'a> {
    pub fn new(
        ctx: &'a egui::Context,
        message: &'a String,
        close_dialog: &'a mut dyn FnMut(),
    ) -> Self {
        Self {
            ctx,
            message,
            close_dialog,
        }
    }
    pub fn show(&mut self) {
        egui::Window::new("Message").show(self.ctx, |ui| {
            ui.label(self.message);
            if ui.button("Ok").clicked() {
                (self.close_dialog)();
            }
        });
    }
}

pub fn show_message_dialog(ctx: &egui::Context, message: &String, close_dialog: &mut dyn FnMut()) {
    let mut dialog = MessageDialog::new(ctx, message, close_dialog);
    dialog.show();
}
