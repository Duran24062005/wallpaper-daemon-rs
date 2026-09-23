use std::path::Path;

mod wallpapers;

fn main() {
    let wallpaper = wallpapers::get_current_wallpaper()
        .expect("Failed to get current wallpaper");

    println!("Current wallpaper: {}", wallpaper);
        
    wallpapers::set_wallpaper(Path::new("/home/alexi-dg/Desktop/wallpaper-rs/assets/2.jpeg")).expect("Failed to set wallpaper.");


}
