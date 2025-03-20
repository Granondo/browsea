use crate::browser::{self};
use crate::config::Config;
use eframe::{egui, epaint};
use crate::theme::Theme;
use crate::icons;

pub struct BrowserPicker {
    pub browsers: Vec<(String, String, Option<egui::TextureHandle>)>,
    pub url: String,
    pub config: Config,
    pub show_settings: bool,
    pub theme: Theme,
    pub dark_mode: bool,
}

impl BrowserPicker {
    pub fn new(cc: &eframe::CreationContext<'_>, url: String) -> Self {
        // Initialize fonts
        Self::init_custom_fonts(&cc.egui_ctx);
        
        let mut picker = Self {
            browsers: Vec::new(),
            url,
            config: Config::load(),
            show_settings: false,
            theme: Theme::light(),
            dark_mode: false,
        };
        
        // Get browsers without icons first
        let mut browsers_info = browser::get_installed_browsers();
        
        // Add custom browsers from config
        browsers_info.extend(picker.config.custom_browsers.clone());
        
        // Load icons for each browser
        for (name, path) in browsers_info {
            let icon = icons::load_browser_icon(&name, &path, &cc.egui_ctx);
            picker.browsers.push((name, path, icon));
        }
        
        picker
    }
    
    fn init_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        
        // Try to load the font from various locations
        let font_data = if let Ok(data) = std::fs::read("src/assets/fonts/segoe-ui.ttf") {
            data
        } else if let Ok(data) = std::fs::read("assets/fonts/segoe-ui.ttf") {
            data
        } else if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                if let Ok(data) = std::fs::read(exe_dir.join("src/assets/fonts/segoe-ui.ttf")) {
                    data
                } else if let Ok(data) = std::fs::read(exe_dir.join("assets/fonts/segoe-ui.ttf")) {
                    data
                } else {
                    // Fallback to included font
                    include_bytes!("assets/fonts/segoe-ui.ttf").to_vec()
                }
            } else {
                // Fallback to included font
                include_bytes!("assets/fonts/segoe-ui.ttf").to_vec()
            }
        } else {
            // Fallback to included font
            include_bytes!("assets/fonts/segoe-ui.ttf").to_vec()
        };
        
        fonts.font_data.insert(
            "segoe".to_owned(),
            egui::FontData::from_owned(font_data),
        );
        
        fonts.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "segoe".to_owned());
        
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for BrowserPicker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let theme = if self.dark_mode { Theme::dark() } else { Theme::light() };
        
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals {
            window_rounding: 8.0.into(),
            window_shadow: epaint::Shadow {
                extrusion: 16.0,
                color: egui::Color32::from_black_alpha(40),
            },
            panel_fill: theme.background,
            window_fill: theme.background,
            faint_bg_color: theme.secondary,
            extreme_bg_color: theme.background,
            code_bg_color: theme.background,
            warn_fg_color: egui::Color32::from_rgb(243, 159, 94),
            error_fg_color: egui::Color32::from_rgb(240, 80, 80),
            window_stroke: egui::Stroke::NONE,
            widgets: egui::style::Widgets::default(),
            selection: egui::style::Selection::default(),
            ..Default::default()
        };
        
        ctx.set_style(style);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_settings {
                self.show_settings_ui(ui);
            } else {
                self.show_browser_picker_ui(ui, frame);
            }
        });
    }
}