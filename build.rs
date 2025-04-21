use std::io;
use std::path::Path;
use winres::WindowsResource;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        
        // Set the manifest file
        res.set_manifest_file("browsea.exe.manifest");
        
        // Set the application icon
        // Try different possible locations for the icon
        let icon_paths = [
            "assets/app_icon/app_icon.ico",
            "src/assets/app_icon/app_icon.ico",
        ];
        
        for icon_path in icon_paths.iter() {
            if Path::new(icon_path).exists() {
                res.set_icon(icon_path);
                println!("cargo:warning=Using icon from: {}", icon_path);
                break;
            }
        }
        
        res.compile()?;
    }
    Ok(())
}
