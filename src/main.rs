mod scanner;
mod selector;
mod wallpapers;
mod scheduler;
mod error;

use error::WallpaperError;


fn run_cycle() -> Result<(), WallpaperError>{
    println!("Changing wallpaper...");
        
    let images = scanner::scan_images("assets").map_err(|error| WallpaperError::Scan(error.to_string()))?;
    
    let current = wallpapers::get_current_wallpaper()?;
    
    let image = selector::select_random(
        &images, 
        Some(current.as_path())
    ).ok_or(WallpaperError::NoAlternative)?;
    
    println!("Current wallpaper: {}", current.display());
    println!("Selected wallpaper: {}", image.display());
    
    wallpapers::set_wallpaper(image)?;

    Ok(())
}

fn main() {
    println!("PATH = {:?}", std::env::var("PATH"));

    let result = std::process::Command::new("/usr/bin/gsettings")
        .arg("--version")
        .output();

    println!("gsettings result = {:?}", result);
    loop {
        if let Err(error) = run_cycle(){
            eprintln!("Wallpaper error: {error}");
        }
        scheduler::wait(10);
    }
}