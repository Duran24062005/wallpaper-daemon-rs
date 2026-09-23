mod wallpapers;
mod scanner;
mod selector;

fn main() {

    let images = scanner::scan_images("assets")
        .expect("Failed to scan images");

    let image = selector::select_random(&images)
        .expect("No images found");

    println!("Selected wallpaper: {}", image.display());

    wallpapers::set_wallpaper(image)
        .expect("Failed to set wallpaper");

    let current = wallpapers::get_current_wallpaper()
        .expect("Failed to get current wallpaper");

    println!("Current wallpaper: {current}");
}
