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

            let browser_count = &self.browsers.len();
            let browser_width = 56.0;
            let total_width = browser_width * (*browser_count as f32);

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        let available_width = ui.available_width();
                        let padding = (available_width - total_width) / 2.0;
                        if padding > 0.0 {
                            ui.add_space(padding);
                        }
                        
                        for (name, _, icon) in &self.browsers {
                            let is_visible = !self.config.hidden_browsers.contains(name);
                            let mut visible = is_visible;

                            ui.horizontal_centered(|ui| {
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    // Show browser icon
                                    if let Some(icon) = icon {
                                        ui.add_sized(
                                            egui::vec2(24.0, 24.0),
                                            egui::Image::new(icon, egui::vec2(20.0, 20.0))
                                        );
                                        ui.add_space(8.0);
                                    }

                                    ui.checkbox(&mut visible, "");
                                });

                                if visible != is_visible {
                                    if visible {
                                        self.config.hidden_browsers.retain(|n| n != name);
                                    } else {
                                        self.config.hidden_browsers.push(name.clone());
                                    }
                                    self.config.save().ok();
                                }
                            });
                        }
                    });
                },
            );
            
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





