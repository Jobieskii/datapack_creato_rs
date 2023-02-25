use eframe::egui::Ui;
use strum::{IntoEnumIterator};

use crate::file::{Window, WindowType};

pub trait ComboBoxEnum : IntoEnumIterator + AsRef<str> + PartialEq + Clone{
    fn show_ui(ui: &mut Ui, current_value: &mut Self) {
        for value in Self::iter() {
            ui.selectable_value(current_value, value.clone(), value.as_ref());
        }
    }
}
pub struct NewWindowPrompt {
    pub show: bool,
    pub namespace: String,
    pub name: String,
    pub window_type: WindowType
}
impl NewWindowPrompt {
    pub fn make_window(&self) -> Window{
        Window::new(self.name.clone(), self.namespace.clone(), self.window_type)
    }
    //TODO: regex match potential namespace + path
    pub fn are_strings_correct (&self) -> bool{
        !self.namespace.is_empty() && !self.name.is_empty() && self.namespace.is_ascii() && self.name.is_ascii()
    }
    pub fn new() -> Self{
        Self {
            show: true,
            namespace: String::new(),
            name: String::new(),
            window_type: WindowType::DensityFunction
        }
    }
    pub fn reset(&mut self) {
        self.namespace.clear();
        self.name.clear();
    }
}