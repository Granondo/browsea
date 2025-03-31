use crate::app::BrowserPicker;
use eframe::egui;
use rfd::FileDialog;

impl BrowserPicker {
    pub fn show_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading(egui::RichText::new("Settings")
                .size(24.0)
                .color(self.theme.foreground));
            
            ui.add_space(16.0);
            
            // Theme toggle
            ui.checkbox(&mut self.dark_mode, "Dark Mode");
            
            ui.add_space(16.0);
            
            // Browser visibility section
            ui.heading(egui::RichText::new("Visible Browsers")
                .size(16.0)
                .color(self.theme.foreground));
            
            ui.add_space(8.0);
            
            for (name, _, icon) in &self.browsers {
                let is_visible = !self.config.hidden_browsers.contains(name);
                let mut visible = is_visible;

                ui.horizontal(|ui| {
                    // Show browser icon
                    if let Some(icon) = icon {
                        ui.add_sized(
                            egui::vec2(24.0, 24.0),
                            egui::Image::new(icon, egui::vec2(16.0, 16.0))
                        );
                        ui.add_space(8.0);
                    }

                    if ui.checkbox(&mut visible, name).changed() {
                        if visible {
                            self.config.hidden_browsers.retain(|n| n != name);
                        } else {
                            self.config.hidden_browsers.push(name.clone());
                        }
                        self.config.save().ok();
                    }
                });
            }
            
            ui.add_space(16.0);
            
            // Add browser button
            if ui.button("➕ Add Custom Browser").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Executable", &["exe"])
                    .set_title("Select Browser Executable")
                    .pick_file() 
                {
                    if let Some(file_name) = path.file_stem() {
                        let browser_name = file_name.to_string_lossy().to_string();
                        let browser_path = path.to_string_lossy().to_string();
                        self.browsers.push((browser_name.clone(), browser_path.clone(), None));
                        self.config.custom_browsers.push((browser_name, browser_path));
                        self.config.save().ok();
                    }
                }
            }
        
            ui.add_space(10.0);
            
            // Save and back buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.config.save().ok();
                }
                
                if ui.button("Back").clicked() {
                    self.show_settings = false;
                }
            });
        });
    }
}

