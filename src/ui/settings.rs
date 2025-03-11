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
            
            // Default browser setting
            ui.label("Default Browser:");
            egui::ComboBox::from_id_source("default_browser")
                .selected_text(self.config.default_browser.clone().unwrap_or_else(|| "None".to_string()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.config.default_browser, None, "None");
                    for (name, _, _) in &self.browsers {
                        ui.selectable_value(&mut self.config.default_browser, Some(name.clone()), name);
                    }
                });
            
            // Remember choice setting
            ui.checkbox(&mut self.config.remember_choice_for_domain, "Remember browser choice for each website");
            
            ui.add_space(10.0);
            
            // Domain preferences
            if !self.config.domain_preferences.is_empty() {
                ui.label("Domain Preferences:");
                ui.add_space(5.0);
                
                let mut domains_to_remove = Vec::new();
                
                for (domain, browser) in &self.config.domain_preferences {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}: {}", domain, browser));
                        if ui.button("Remove").clicked() {
                            domains_to_remove.push(domain.clone());
                        }
                    });
                }
                
                for domain in domains_to_remove {
                    self.config.domain_preferences.remove(&domain);
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