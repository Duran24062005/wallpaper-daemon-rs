use std::path::Path;

use crate::wallpapers::set_wallpaper;

mod wallpapers;

fn main() {

        let wallpaper = wallpapers::get_current_wallpaper().expect("Failed to get current path");

        println!("Current wallpaper: {wallpaper}");
        
        set_wallpaper(Path::new("/home/alexi-dg/Desktop/wallpaper-rs/assets/4.jpeg")).expect("Failed to set wallpaper.");


}