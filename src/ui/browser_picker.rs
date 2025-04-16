use crate::app::BrowserPicker;
use crate::browser_launcher;
use eframe::egui;

impl BrowserPicker {
    pub fn show_browser_picker_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            // Add settings button at the top right
            ui.horizontal(|ui| {
                // Push the settings icon to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let circle_size = egui::vec2(32.0, 32.0);
                    let (rect, response) = ui.allocate_exact_size(circle_size, egui::Sense::click());

                    if ui.is_rect_visible(rect) {
                        // Draw circular background
                        let circle_color = if ui.rect_contains_pointer(rect) {
                            self.theme.button_hover
                        } else {
                            self.theme.button_bg
                        };
                        
                        ui.painter().circle(
                            rect.center(),
                            rect.width() / 2.0,
                            circle_color,
                            egui::Stroke::new(1.0, self.theme.button_bg)
                        );

                        // Draw settings icon (⚙) centered in the circle
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "⚙",
                            egui::FontId::proportional(20.0),
                            self.theme.foreground
                        );
                    }

                    if response.clicked() {
                        self.show_settings = true;
                    }
                    ui.add_space(16.0);  // Match the spacing from theme toggle
                });
            });

            ui.add_space(8.0);  // Reduced space after settings button

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
                        
                        for (i, (name, _, icon)) in visible_browsers.iter().enumerate() {
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
                                            egui::vec2(60.0, 60.0),  // Increased button size
                                            egui::ImageButton::new(icon, egui::vec2(48.0, 48.0))  // Display size matches loaded size
                                                .frame(false)
                                        )
                                    }).inner
                                } else {
                                    ui.add(egui::Button::new(
                                        egui::RichText::new(name)
                                            .size(16.0)
                                            .color(self.theme.foreground)
                                    )
                                    .min_size(egui::vec2(72.0, 72.0))
                                    .rounding(12.0)
                                    .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                                        self.theme.button_hover
                                    } else {
                                        self.theme.button_bg
                                    }))
                                };
                                
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
        });
    }
}



















