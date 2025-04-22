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
                egui::RichText::new("BROWSERS")
                    .size(36.0)
                    .color(self.theme.primary)
            );

            ui.add_space(8.0); // Reduced space after heading

            // Fixed width container for browser list
            let browser_list_width = 280.0;
            egui::Frame::none()
                .inner_margin(egui::style::Margin::symmetric(0.0, 0.0))
                .show(ui, |ui| {
                    ui.set_width(browser_list_width);
                    
                    // Create an index-based iteration
                    let browser_count = self.browsers.len();
                    let mut browsers_to_remove = Vec::new();

                    for i in 0..browser_count {
                        let (name, _, icon) = &self.browsers[i];
                        let name = name.clone(); // Clone the name for use in closures
                        let is_visible = !self.config.hidden_browsers.contains(&name);
                        let mut visible = is_visible;

                        ui.horizontal(|ui| {
                            // Icon
                            if let Some(icon) = icon {
                                ui.add_sized(
                                    egui::vec2(48.0, 48.0),
                                    egui::Image::new(icon, egui::vec2(40.0, 40.0))
                                );
                            }

                            // Name
                            let label = egui::Label::new(
                                egui::RichText::new(&name)
                                    .size(14.0)
                                    .color(self.theme.foreground)
                            );
                            ui.add(label);
                            
                            // Push checkbox and delete button to the right
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Delete button for all browsers
                                let delete_size = egui::vec2(18.0, 18.0);
                                let (rect, delete_response) = ui.allocate_exact_size(delete_size, egui::Sense::click());

                                if ui.is_rect_visible(rect) {
                                    // Draw circle background based on theme
                                    let circle_color = if ui.rect_contains_pointer(rect) {
                                        self.theme.button_hover
                                    } else {
                                        self.theme.button_bg
                                    };
                                    
                                    ui.painter().circle(
                                        rect.center(),
                                        rect.width() / 2.0,
                                        circle_color,
                                        egui::Stroke::NONE,
                                    );

                                    // Draw red X
                                    let cross_color = egui::Color32::from_rgb(239, 68, 68); // Red color
                                    let padding = 5.0;
                                    ui.painter().line_segment(
                                        [
                                            rect.min + egui::vec2(padding, padding),
                                            rect.max - egui::vec2(padding, padding)
                                        ],
                                        egui::Stroke::new(2.0, cross_color)
                                    );
                                    ui.painter().line_segment(
                                        [
                                            egui::pos2(rect.min.x + padding, rect.max.y - padding),
                                            egui::pos2(rect.max.x - padding, rect.min.y + padding)
                                        ],
                                        egui::Stroke::new(2.0, cross_color)
                                    );
                                }

                                if delete_response.clicked() {
                                    browsers_to_remove.push(i);
                                }

                                ui.add_space(8.0); // Space between delete button and checkbox

                                // Checkbox
                                let checkbox_size = egui::vec2(18.0, 18.0);
                                let (rect, response) = ui.allocate_exact_size(checkbox_size, egui::Sense::click());

                                if response.clicked() {
                                    visible = !visible;
                                }

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
                                        let check_color = self.theme.background;
                                        let points = [
                                            rect.min + egui::vec2(4.0, 9.0),
                                            rect.min + egui::vec2(8.0, 13.0),
                                            rect.min + egui::vec2(14.0, 5.0),
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
                            });
                        });

                        if visible != is_visible {
                            if visible {
                                self.config.hidden_browsers.retain(|n| n != &name);
                            } else {
                                self.config.hidden_browsers.push(name.clone());
                            }
                            self.config.save().ok();
                        }

                        ui.add_space(2.0);
                    }

                    // Process removals after the loop
                    if !browsers_to_remove.is_empty() {
                        // Remove in reverse order to maintain correct indices
                        for &i in browsers_to_remove.iter().rev() {
                            let (name, _, _) = &self.browsers[i];
                            let name = name.clone();
                            self.browsers.remove(i);
                            self.config.hidden_browsers.retain(|n| n != &name);
                        }
                        self.config.save().ok();
                    }
                });

            ui.add_space(16.0);

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



