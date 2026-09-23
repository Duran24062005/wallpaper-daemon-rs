use std::fs;
use std::path::PathBuf;


pub fn scan_images(directory: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                let extension = extension.to_string_lossy().to_lowercase();

                if matches!(extension.as_str(), "jpg" | "jpeg" | "png" | "webp") {
                    images.push(path);
                }
            }
        }
    }

    Ok(images)
}