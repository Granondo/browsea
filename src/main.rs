#![windows_subsystem = "windows"]

mod app;
mod browser;
mod config;
mod registry;
mod theme;
mod ui;

use app::BrowserPicker;
use eframe::egui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // Show browser picker for URL
        let url = args[1].clone();
        let options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(200.0, 300.0)),
            centered: true,
            always_on_top: true,
            decorated: true,
            transparent: false,
            ..Default::default()
        };
        
        // Handle the Result returned by run_native
        if let Err(e) = eframe::run_native(
            "Browser Picker",
            options,
            Box::new(|cc| Box::new(BrowserPicker::new(cc, url))),
        ) {
            eprintln!("Failed to run application: {}", e);
            std::process::exit(1);
        }
    } else {
        // Register as browser handler
        if let Err(e) = registry::register_browser() {
            eprintln!("Failed to register browser: {}", e);
            std::process::exit(1);
        }
    }
}