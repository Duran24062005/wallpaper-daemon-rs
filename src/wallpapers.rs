use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::WallpaperError;

const GSETTINGS: &str = "/usr/bin/gsettings";

pub fn get_current_wallpaper() -> Result<PathBuf, WallpaperError> {
    let output = Command::new(GSETTINGS)
        .args([
            "get", 
            "org.gnome.desktop.background", 
            "picture-uri-dark"
        ])
        .output().map_err(|source| WallpaperError::Command {
        command: "gsettings",
        source,
    })?;

    if !output.status.success() {
        return Err(WallpaperError::GSettings { 
            operation: "get", 
            status: output.status 
        });
    }

    let wallpaper = String::from_utf8(output.stdout)?;

    let wallpaper = wallpaper.trim().trim_matches('\'');

    let path = wallpaper.strip_prefix("file://").ok_or_else(|| WallpaperError::InvalidUri(wallpaper.to_string()))?;

    Ok(Path::new(path).canonicalize().map_err(|source| WallpaperError::Command {
        command: "gsettings",
        source,
    })?)
}

pub fn set_wallpaper(image: &Path) -> Result<(), WallpaperError> {
    let uri = format!("file://{}", image.canonicalize().map_err(|source| WallpaperError::Command {
        command: "gsettings",
        source,
    })?.display());

    let status = Command::new(GSETTINGS)
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &uri,
        ])
        .status().map_err(|source| WallpaperError::Command {
        command: "gsettings",
        source,
    })?;

    if !status.success() {
        return Err(WallpaperError::GSettings { 
            operation: "set", 
            status
        });
    }

    Ok(())
}
