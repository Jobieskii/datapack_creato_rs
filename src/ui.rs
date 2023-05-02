use std::path::PathBuf;

use eframe::egui::{Button, Response, Ui};
use strum::IntoEnumIterator;

use crate::window::{Window, WindowType};

pub trait ComboBoxEnum: IntoEnumIterator + AsRef<str> + PartialEq + Clone {
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
    pub window_type: WindowType,
}
impl NewWindowPrompt {
    pub fn make_window(&self, project_path: &PathBuf) -> Window {
        Window::new(
            self.name.clone(),
            self.namespace.clone(),
            self.window_type,
            project_path,
        )
    }
    //TODO: regex match potential namespace + path
    pub fn are_strings_correct(&self) -> bool {
        !self.namespace.is_empty()
            && !self.name.is_empty()
            && self.namespace.is_ascii()
            && self.name.is_ascii()
    }
    pub fn new(show: bool) -> Self {
        Self {
            show,
            namespace: String::new(),
            name: String::new(),
            window_type: WindowType::DensityFunction,
        }
    }
    pub fn reset(&mut self) {
        self.namespace.clear();
        self.name.clear();
    }
}

pub struct OpenProjectPrompt {
    pub show: bool,
    pub path: String,
    // TODO: project creation with name, description, version
}
impl OpenProjectPrompt {
    pub fn new(show: bool) -> Self {
        Self {
            show,
            path: String::new(),
        }
    }
    pub fn ui_entered(&mut self, ui: &mut Ui) -> bool {
        ui.label("Open project folder");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.path);
            if ui.small_button("find").clicked() {
                if let Some(path) = tinyfiledialogs::select_folder_dialog("Project Path", "") {
                    self.path = path;
                }
            }
        });
        if ui
            .add_enabled(!self.path.is_empty(), Button::new("load"))
            .clicked()
        {
            true
        } else {
            false
        }
    }
}
