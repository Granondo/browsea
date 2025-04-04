#![windows_subsystem = "windows"]

mod app;
mod browser;
mod browser_launcher;
mod config;
mod registry;
mod theme;
mod ui;
mod icons;

use app::BrowserPicker;
use eframe::egui;
use std::path::PathBuf;

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
            icon_data: load_icon(),
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

fn load_icon() -> Option<eframe::IconData> {
    // Try multiple possible locations for the app icon
    let possible_paths = vec![
        PathBuf::from("src/assets/app_icon/app_icon.png"),
        PathBuf::from("assets/app_icon/app_icon.png"),
    ];

    // Add paths relative to the executable location
    let mut exe_paths = Vec::new();
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            exe_paths.push(exe_dir.join("src/assets/app_icon/app_icon.png"));
            exe_paths.push(exe_dir.join("assets/app_icon/app_icon.png"));
        }
    }

    // Try to find and load the icon from any of the possible locations
    for path in possible_paths.iter().chain(exe_paths.iter()) {
        if let Ok(image) = image::open(path) {
            println!("Successfully loaded icon image from: {:?}", path);
            let image = image.to_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            return Some(eframe::IconData {
                rgba,
                width,
                height,
            });
        }
    }

    eprintln!("Failed to load application icon from any location");
    None
}