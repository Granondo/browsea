use crate::app::BrowserPicker;
use crate::browser_launcher;
use eframe::egui;

impl BrowserPicker {
    pub fn show_browser_picker_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading(egui::RichText::new("Choose Browser")
                .size(24.0)
                .color(self.theme.foreground));
            
            ui.add_space(16.0);
            
            // URL display
            ui.label(egui::RichText::new(&self.url)
                .size(14.0)
                .color(self.theme.secondary));
            
            ui.add_space(16.0);
            
            // Browser buttons with icons
            ui.horizontal_wrapped(|ui| {
                for (i, (name, path, icon)) in self.browsers.iter().enumerate() {
                    let browser_index = i; // Store index for click handler
                    ui.vertical(|ui| {
                        let mut response = if let Some(icon) = icon {
                            // Use icon if available
                            ui.add_sized(
                                egui::vec2(48.0, 48.0),
                                egui::ImageButton::new(icon, egui::vec2(32.0, 32.0))
                                    .frame(true)
                            )
                        } else {
                            // Fallback to text button
                            ui.add(egui::Button::new(
                                egui::RichText::new(name)
                                    .size(16.0)
                                    .color(self.theme.foreground)
                            )
                            .min_size(egui::vec2(48.0, 48.0))
                            .rounding(8.0)
                            .fill(self.theme.background))
                        };
                        
                        // Add browser name below the icon
                        ui.label(egui::RichText::new(name)
                            .size(12.0)
                            .color(self.theme.foreground));
                        
                        if response.hovered() {
                            response.mark_changed();
                            ui.painter().rect_filled(
                                response.rect,
                                8.0,
                                if self.dark_mode {
                                    egui::Color32::from_white_alpha(10)
                                } else {
                                    egui::Color32::from_black_alpha(10)
                                },
                            );
                        }
                        
                        // Handle click event to launch browser
                        if response.clicked() {
                            if let Some((_, browser_path, _)) = self.browsers.get(browser_index) {
                                if let Err(e) = browser_launcher::launch_browser(browser_path, &self.url) {
                                    eprintln!("{}", e);
                                }
                                frame.close();
                            }
                        }
                    });
                    
                    ui.add_space(8.0); // Space between browser icons
                }
            });
            
            ui.add_space(16.0);
            
            // Settings button
            if ui.button(
                egui::RichText::new("⚙ Settings")
                    .size(14.0)
                    .color(self.theme.secondary)
            ).clicked() {
                self.show_settings = true;
            }
        });
    }
}