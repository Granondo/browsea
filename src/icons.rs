use std::path::Path;
use eframe::egui;
use std::env;

pub fn get_browser_icon_path(browser_name: &str) -> Option<String> {
    // Normalize the browser name for matching
    let name = browser_name.to_lowercase();
    
    // Match common browsers to their icon files
    let icon_name = if name.contains("chrome") {
        "chrome.png"
    } else if name.contains("firefox") || name.contains("mozilla") {
        "firefox.png"
    } else if name.contains("edge") {
        "edge.png"
    } else if name.contains("opera") {
        "opera.png"
    } else if name.contains("safari") {
        "safari.png"
    } else if name.contains("brave") {
        "brave.png"
    } else if name.contains("internet explorer") || name.contains("iexplore") {
        "ie.png"
    } else {
        return None;
    };
    
    // Return the path to the icon
    Some(format!("src/assets/browser_icons/{}", icon_name))
}

fn find_icon_file(base_path: &str) -> Option<String> {
    // Try different possible locations for the assets
    let possible_paths = vec![
        format!("{}", base_path),
        format!("src/assets/browser_icons/{}", Path::new(base_path).file_name().unwrap().to_str().unwrap()),
        format!("assets/browser_icons/{}", Path::new(base_path).file_name().unwrap().to_str().unwrap()),
    ];
    
    // Check if we're running from the installation directory
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let installed_path1 = exe_dir.join("src/assets/browser_icons")
                .join(Path::new(base_path).file_name().unwrap());
            let installed_path2 = exe_dir.join("assets/browser_icons")
                .join(Path::new(base_path).file_name().unwrap());
            
            possible_paths.iter().chain([
                installed_path1.to_string_lossy().to_string(),
                installed_path2.to_string_lossy().to_string(),
            ].iter()).find(|path| Path::new(path).exists()).cloned()
        } else {
            possible_paths.iter().find(|path| Path::new(path).exists()).cloned()
        }
    } else {
        possible_paths.iter().find(|path| Path::new(path).exists()).cloned()
    }
}

pub fn load_browser_icon(browser_name: &str, _path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    // First try to load from bundled assets based on browser name
    if let Some(icon_path) = get_browser_icon_path(browser_name) {
        // Try to find the icon file in various locations
        if let Some(found_path) = find_icon_file(&icon_path) {
            // Load the icon from the file
            if let Ok(image) = image::open(&found_path) {
                let image = image.resize(32, 32, image::imageops::FilterType::Lanczos3);
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_raw().to_vec(); // Create owned vector from the slice
                
                return Some(ctx.load_texture(
                    format!("browser_icon_{}", browser_name),
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    egui::TextureOptions::default(),
                ));
            }
        }
    }
    
    // If no matching icon or loading failed, create a fallback colored icon
    create_fallback_icon(browser_name, ctx)
}

fn create_fallback_icon(browser_name: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // Generate a deterministic color based on the browser name
    let mut hasher = DefaultHasher::new();
    browser_name.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Extract RGB components from hash
    let r = ((hash >> 16) & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = (hash & 0xFF) as u8;
    
    // Create a colored square as fallback
    let size = 32;
    let mut pixels = vec![0; size * size * 4];
    
    // Fill with the generated color
    for i in 0..size * size {
        pixels[i * 4] = r;
        pixels[i * 4 + 1] = g;
        pixels[i * 4 + 2] = b;
        pixels[i * 4 + 3] = 255; // Alpha
    }
    
    // Create texture from pixels
    let texture = ctx.load_texture(
        format!("browser_icon_{}", browser_name),
        egui::ColorImage::from_rgba_unmultiplied([size as _, size as _], &pixels),
        egui::TextureOptions::default(),
    );
    
    Some(texture)
}
