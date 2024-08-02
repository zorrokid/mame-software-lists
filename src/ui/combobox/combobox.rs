use crate::ui::selection_options::SelectionOptions;
use eframe::egui;

pub struct ComboBox<'a, T>
where
    T: Clone,
{
    ui: &'a mut egui::Ui,
    selection_options: &'a SelectionOptions<T>,
    on_selected_changed: &'a mut dyn FnMut(Option<T>),
    label: &'a String,
}

impl<'a, T> ComboBox<'a, T>
where
    T: Clone + std::fmt::Display + std::cmp::PartialEq,
{
    pub fn new(
        ui: &'a mut egui::Ui,
        selection_options: &'a SelectionOptions<T>,
        on_selected_changed: &'a mut dyn FnMut(Option<T>),
        label: &'a String,
    ) -> Self {
        Self {
            ui,
            selection_options,
            on_selected_changed,
            label,
        }
    }

    pub fn show(&mut self) {
        let mut options = self.selection_options.clone();
        egui::ComboBox::from_label(self.label)
            .selected_text(
                self.selection_options
                    .selected
                    .as_ref()
                    .map_or("Select an item".to_string(), |item| item.to_string()),
            )
            .show_ui(self.ui, |ui| {
                for item in options.items.iter() {
                    if ui
                        .selectable_value(
                            &mut options.selected,
                            Some(item.clone()),
                            item.to_string(),
                        )
                        .clicked()
                    {
                        (self.on_selected_changed)(options.selected.clone());
                    }
                }
            });
    }
}

pub fn show_combobox<T>(
    ui: &mut egui::Ui,
    selection_options: &SelectionOptions<T>,
    on_selected_changed: &mut dyn FnMut(Option<T>),
    label: &String,
) where
    T: Clone + std::fmt::Display + std::cmp::PartialEq,
{
    let mut combobox = ComboBox::<T>::new(ui, selection_options, on_selected_changed, label);
    combobox.show();
}
