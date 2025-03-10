use eframe::egui;

pub struct Theme {
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub primary: egui::Color32,
    pub secondary: egui::Color32,
    pub accent: egui::Color32,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            background: egui::Color32::from_rgb(245, 245, 245),
            foreground: egui::Color32::from_rgb(32, 32, 32),
            primary: egui::Color32::from_rgb(59, 130, 246),
            secondary: egui::Color32::from_rgb(107, 114, 128),
            accent: egui::Color32::from_rgb(99, 102, 241),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: egui::Color32::from_rgb(32, 32, 32),
            foreground: egui::Color32::from_rgb(245, 245, 245),
            primary: egui::Color32::from_rgb(59, 130, 246),
            secondary: egui::Color32::from_rgb(156, 163, 175),
            accent: egui::Color32::from_rgb(129, 140, 248),
        }
    }
} 