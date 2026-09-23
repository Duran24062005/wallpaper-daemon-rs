use std::path::Path;
use std::process::Command;

pub fn get_current_wallpaper() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("gsettings")
        .args([
            "get",
            "org.gnome.desktop.background",
            "picture-uri",
        ])
        .output()?;

    let wallpaper = String::from_utf8(output.stdout)?;

    let wallpaper = wallpaper.trim().trim_matches('\'');

    Ok(wallpaper.to_string())
}

pub fn set_wallpaper(image: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("file://{}", image.canonicalize()?.display());

    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &uri,
        ])
        .status()?;

    Ok(())
}
