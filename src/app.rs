use crate::browser::{self, extract_domain};
use crate::config::Config;
use eframe::{egui, epaint};
use crate::theme::Theme;

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
            let icon = Self::load_browser_icon(&path, &cc.egui_ctx);
            picker.browsers.push((name, path, icon));
        }
        
        picker
    }
    
    fn init_custom_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "segoe".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/segoe-ui.ttf")),
        );
        
        fonts.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "segoe".to_owned());
        
        ctx.set_fonts(fonts);
    }
    
    fn load_browser_icon(_path: &str, _ctx: &egui::Context) -> Option<egui::TextureHandle> {
        None
    }

    pub fn check_domain_preference(&self) -> Option<String> {
        if self.config.remember_choice_for_domain {
            if let Some(domain) = extract_domain(&self.url) {
                return self.config.domain_preferences.get(&domain).cloned();
            }
        }
        self.config.default_browser.clone()
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