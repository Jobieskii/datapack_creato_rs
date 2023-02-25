#![feature(once_cell)] 
use app::App;

mod app;
mod nodes;
mod file;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "My egui App", 
        native_options, 
        Box::new(
            |cc| {
                Box::new(App::new(cc))
            }
        )
    );
}
