// #![feature(once_cell)] 
use app::App;
use std::path::PathBuf;
use simple_logger::SimpleLogger;
mod app;
mod nodes;
mod window;
mod ui;
mod serializer;
mod errors;

fn main() {
    // TODO: Rather than just logging error messages, display them in the UI (global queue of messages?)
    SimpleLogger::new().init().unwrap();
    let native_options = eframe::NativeOptions::default();
    let path = std::env::args().nth(1).map(|s| PathBuf::from(s));
    eframe::run_native(
        "Datapack creato(rs)", 
        native_options, 
        Box::new(
            |cc| {
                Box::new(App::new(cc, path))
            }
        )
    );
}
