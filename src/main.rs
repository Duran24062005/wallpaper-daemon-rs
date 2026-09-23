use std::path::Path;

mod wallpapers;
mod scanner;

fn main() {

    let images = scanner::scan_images("assets").expect("Failed to scan images");

    for image in images {
        println!("{}", image.display());
    }


    let wallpaper = wallpapers::get_current_wallpaper()
        .expect("Failed to get current wallpaper");

    println!("Current wallpaper: {}", wallpaper);
        
    wallpapers::set_wallpaper(Path::new("/home/alexi-dg/Desktop/wallpaper-rs/assets/2.jpeg")).expect("Failed to set wallpaper.");

    let new_wallpaper = wallpapers::get_current_wallpaper()
        .expect("Failed to get current wallpaper");

    println!("New current wallpaper: {}", new_wallpaper);
}
