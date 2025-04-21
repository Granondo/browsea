use crate::app::Browsea;
use eframe::egui;
use rfd::FileDialog;

impl Browsea {
    pub fn show_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            // Header row with back button and theme toggle
            ui.horizontal(|ui| {
                // Back button on the left
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let icon_size = egui::vec2(24.0, 24.0);
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

                        // Draw back arrow
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "⬅"  ,  // Simple arrow, will be flipped with scale
                            egui::FontId::proportional(20.0),
                            self.theme.foreground
                        );
                    }

                    if response.clicked() {
                        self.show_settings = false;
                    }
                });

                // Push the theme toggle to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let icon_size = egui::vec2(20.0, 20.0);  // Keep icon size at 24.0
                    let circle_size = egui::vec2(32.0, 32.0); // New variable for circle size
                    let (rect, response) = ui.allocate_exact_size(circle_size, egui::Sense::click());  // Use circle_size here

                    if ui.is_rect_visible(rect) {
                        // Draw circular background
                        let circle_color = if ui.rect_contains_pointer(rect) {
                            self.theme.button_hover
                        } else {
                            self.theme.button_bg
                        };
                        
                        ui.painter().circle(
                            rect.center(),
                            rect.width() / 2.0,  // This will now use the circle_size width
                            circle_color,
                            egui::Stroke::new(1.0, self.theme.button_bg)
                        );

                        // Calculate centered position for the icon within the larger circle
                        let icon_rect = egui::Rect::from_center_size(rect.center(), icon_size);

                        // Draw the icon
                        if self.dark_mode {
                            if let Some(icon) = &self.sun_icon {
                                ui.put(icon_rect, egui::Image::new(icon, icon_size));
                            }
                        } else {
                            if let Some(icon) = &self.moon_icon {
                                ui.put(icon_rect, egui::Image::new(icon, icon_size));
                            }
                        }
                    }

                    if response.clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                    ui.add_space(16.0);
                });
            });

            ui.add_space(24.0);

            // Browser visibility section
            ui.heading(
                egui::RichText::new("BROWSERS")  // Using uppercase for more emphasis
                    .size(36.0)
                    // .family(egui::FontFamily::Monospace)  // Using monospace font
                    // .strong()
                    // .raised()
                    .color(self.theme.primary)
            );


            ui.add_space(16.0);

            let browser_count = &self.browsers.len();
            let browser_width = 72.0; // Increased for better spacing
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
                                    ui.add_space(8.0); // Consistent left padding

                                    // Show browser icon
                                    if let Some(icon) = icon {
                                        ui.add_sized(
                                            egui::vec2(32.0, 32.0),
                                            egui::Image::new(icon, egui::vec2(24.0, 24.0))
                                        );
                                    }
                                    
                                    // ui.add_space(12.0); // Consistent spacing between icon and checkbox

                                    // Custom styled checkbox
                                    let checkbox_size = egui::vec2(20.0, 20.0);
                                    let (rect, response) = ui.allocate_exact_size(checkbox_size, egui::Sense::click());

                                    if response.clicked() {
                                        visible = !visible;
                                    }

                                    // Draw custom checkbox
                                    if ui.is_rect_visible(rect) {
                                        let stroke = egui::Stroke::new(1.0, self.theme.primary);
                                        let rounding = 4.0;

                                        ui.painter().rect(
                                            rect,
                                            rounding,
                                            if visible { self.theme.primary } else { self.theme.button_bg },
                                            stroke,
                                        );

                                        if visible {
                                            // Draw checkmark
                                            let check_color = self.theme.background;
                                            let points = [
                                                rect.min + egui::vec2(5.0, 10.0),
                                                rect.min + egui::vec2(9.0, 14.0),
                                                rect.min + egui::vec2(15.0, 6.0),
                                            ];
                                            ui.painter().line_segment(
                                                [points[0], points[1]],
                                                egui::Stroke::new(2.0, check_color)
                                            );
                                            ui.painter().line_segment(
                                                [points[1], points[2]],
                                                egui::Stroke::new(2.0, check_color)
                                            );
                                        }
                                    }

                                    ui.add_space(8.0); // Consistent right padding
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

            ui.add_space(24.0);

            // Add browser button with consistent styling
            if ui.add(egui::Button::new(
                egui::RichText::new("➕ Add Custom Browser")
                    .size(14.0)
                    .color(self.theme.foreground)
            )
            .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                self.theme.button_hover
            } else {
                self.theme.button_bg
            })
            .min_size(egui::vec2(200.0, 36.0)) // Consistent button size
            .rounding(8.0)).clicked() {
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

            ui.add_space(16.0);
        });
    }
}



































