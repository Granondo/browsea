use crate::app::Browsea;
use crate::browser_launcher;
use eframe::egui;

impl Browsea {
    pub fn show_browser_picker_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Settings button at the top
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let circle_size = egui::vec2(32.0, 32.0);
                let (rect, response) = ui.allocate_exact_size(circle_size, egui::Sense::click());

                if ui.is_rect_visible(rect) {
                    ui.painter().circle(
                        rect.center(),
                        rect.width() / 2.0,
                        if ui.rect_contains_pointer(rect) {
                            self.theme.button_hover
                        } else {
                            self.theme.button_bg
                        },
                        egui::Stroke::new(1.0, self.theme.button_bg)
                    );

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
                ui.add_space(16.0);
            });
        });

        // Main content area with browser icons
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show_inside(ui, |ui| {
                let visible_browsers: Vec<_> = self.browsers.iter()
                    .filter(|(name, _, _)| !self.config.hidden_browsers.contains(name))
                    .collect();

                // Calculate grid layout
                let available_space = ui.available_size();
                let icon_size = egui::vec2(60.0, 60.0);
                let spacing = 8.0;
                let items_per_row = (available_space.x / (icon_size.x + spacing)).floor().max(1.0) as usize;
                let rows = (visible_browsers.len() as f32 / items_per_row as f32).ceil() as usize;
                
                // Calculate total height needed
                let total_height = rows as f32 * (icon_size.y + spacing);
                let vertical_padding = (available_space.y - total_height) / 2.0;

                // Add top padding
                ui.add_space(vertical_padding);

                // Create grid of browser icons
                egui::Grid::new("browser_grid")
                    .spacing([spacing, spacing])
                    .min_col_width(icon_size.x)
                    .max_col_width(icon_size.x)
                    .show(ui, |ui| {
                        for (idx, (name, path, icon)) in visible_browsers.iter().enumerate() {
                            if idx > 0 && idx % items_per_row == 0 {
                                ui.end_row();
                            }

                            ui.vertical_centered(|ui| {
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
                                            icon_size,
                                            egui::ImageButton::new(icon, egui::vec2(48.0, 48.0))
                                                .frame(false)
                                        )
                                    }).inner
                                } else {
                                    ui.add_sized(
                                        icon_size,
                                        egui::Button::new(
                                            egui::RichText::new(name)
                                                .size(16.0)
                                                .color(self.theme.foreground)
                                        )
                                        .rounding(12.0)
                                        .fill(if ui.rect_contains_pointer(ui.min_rect()) {
                                            self.theme.button_hover
                                        } else {
                                            self.theme.button_bg
                                        })
                                    )
                                };
                                
                                if button_response.clicked() {
                                    if let Err(e) = browser_launcher::launch_browser(path, &self.url) {
                                        eprintln!("{}", e);
                                    }
                                    frame.close();
                                }
                            });
                        }
                    });
            });
    }
}





















