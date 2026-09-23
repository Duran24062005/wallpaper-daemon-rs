mod wallpapers;

fn main() {
    let wallpaper = wallpapers::get_current_wallpaper()
        .expect("Failed to get current wallpaper");

    println!("Current wallpaper: {wallpaper}");
}
