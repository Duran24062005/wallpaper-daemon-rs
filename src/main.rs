use std::path::Path;

mod wallpapers;
mod scanner;
mod selector;

fn main() {
    
    let images = scanner::scan_images("assets").expect("Failed to scan images.");

    let current  = wallpapers::get_current_wallpaper().expect("Failed to get current wallpaper");

    let current_path = current.strip_prefix("file://").map(Path::new);

    let image = selector::select_random(&images, current_path).expect("No alternative wallpapers found.");

    println!("Current wallpaper: {current}");
    println!("Selected wallpaper: {}", image.display());

    wallpapers::set_wallpaper(image).expect("Failed to set wallpaper.");
}
