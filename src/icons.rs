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
    // Try multiple possible locations for the icon
    let possible_paths = vec![
        base_path.to_string(),
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                exe_dir.join(base_path).to_string_lossy().to_string()
            } else {
                base_path.to_string()
            }
        } else {
            base_path.to_string()
        },
    ];

    // Try each path and return the first one that exists
    for path in possible_paths {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}

pub fn load_browser_icon(browser_name: &str, _path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    // First try to load from bundled assets based on browser name
    if let Some(icon_path) = get_browser_icon_path(browser_name) {
        if let Some(found_path) = find_icon_file(&icon_path) {
            println!("Found browser icon at: {}", found_path);
            
            if let Some(image) = load_and_process_image(&found_path) {
                return Some(ctx.load_texture(
                    format!("browser_icon_{}", browser_name),
                    image,
                    egui::TextureOptions::default(),
                ));
            }
        } else {
            println!("Could not find browser icon: {}", browser_name);
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

pub fn load_theme_icon(icon_name: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let icon_path = format!("src/assets/theme_icons/{}.png", icon_name);
    // Try to find the icon file in various locations
    if let Some(found_path) = find_icon_file(&icon_path) {
        println!("Found theme icon at: {}", found_path);

            if let Some(image) = load_and_process_image(&found_path) {
                return Some(ctx.load_texture(
                    format!("theme_icon_{}", icon_name),
                    image,
                    egui::TextureOptions::default(),
                ));
            }
    } else {
        println!("Could not find theme icon: {}", icon_name);
    }

    // Create a fallback icon if the theme icon couldn't be loaded
    create_fallback_icon(icon_name, ctx)
}

fn load_and_process_image(path: &str) -> Option<egui::ColorImage> {
    match image::open(path) {
        Ok(image) => {
            // Increase size from 32 to 64 for better quality when scaling
            let image = image.resize(512, 512, image::imageops::FilterType::Lanczos3);
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_raw().to_vec();
            Some(egui::ColorImage::from_rgba_unmultiplied(size, &pixels))
        },
        Err(_) => {
            println!("Failed to open image at: {}", path);
            None
        }
    }
}



