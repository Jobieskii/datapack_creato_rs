#![feature(once_cell)] 
use app::App;
use std::path::PathBuf;

mod app;
mod nodes;
mod window;
mod ui;
mod serializer;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let path = std::env::args().nth(1).unwrap_or(String::new());
    eframe::run_native(
        "Datapack creato(rs)", 
        native_options, 
        Box::new(
            |cc| {
                Box::new(App::new(cc, PathBuf::from(path)))
            }
        )
    );
}
