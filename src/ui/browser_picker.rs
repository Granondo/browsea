use crate::app::BrowserPicker;
use crate::browser_launcher;
use eframe::egui;

impl BrowserPicker {
    pub fn show_browser_picker_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.add_space(16.0);

            let visible_browsers: Vec<_> = self.browsers.iter()
                .filter(|(name, _, _)| !self.config.hidden_browsers.contains(name))
                .collect();

            let browser_count = visible_browsers.len();
            let browser_width = 56.0;
            let total_width = browser_width * (browser_count as f32);
            
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
                        
                        for (i, (name, path, icon)) in visible_browsers.iter().enumerate() {
                            let browser_index = i;
                            ui.vertical(|ui| {
                                let button_response = if let Some(icon) = icon {
                                    let frame = egui::Frame::none()
                                        .rounding(12.0)
                                        .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                                            self.theme.button_hover
                                        } else {
                                            self.theme.button_bg
                                        });

                                    frame.show(ui, |ui| {
                                        ui.add_sized(
                                            egui::vec2(48.0, 48.0),
                                            egui::ImageButton::new(icon, egui::vec2(32.0, 32.0))
                                                .frame(false)
                                        )
                                    }).inner
                                } else {
                                    ui.add(egui::Button::new(
                                        egui::RichText::new(name)
                                            .size(16.0)
                                            .color(self.theme.foreground)
                                    )
                                    .min_size(egui::vec2(48.0, 48.0))
                                    .rounding(12.0)
                                    .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                                        self.theme.button_hover
                                    } else {
                                        self.theme.button_bg
                                    }))
                                };
                                
                                ui.label(egui::RichText::new(name)
                                    .size(12.0)
                                    .color(self.theme.foreground));
                                
                                if button_response.clicked() {
                                    if let Some((_, browser_path, _)) = self.browsers.get(browser_index) {
                                        if let Err(e) = browser_launcher::launch_browser(browser_path, &self.url) {
                                            eprintln!("{}", e);
                                        }
                                        frame.close();
                                    }
                                }
                            });
                            
                            ui.add_space(8.0);
                        }
                    });
                }
            );
            
            ui.add_space(16.0);
            
            // Settings button with updated styling
            let settings_button = ui.add(egui::Button::new(
                egui::RichText::new("⚙ Settings")
                    .size(14.0)
                    .color(self.theme.foreground)
            )
            .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                self.theme.button_hover
            } else {
                self.theme.button_bg
            })
            .rounding(8.0));
            
            if settings_button.clicked() {
                self.show_settings = true;
            }
        });
    }
}












