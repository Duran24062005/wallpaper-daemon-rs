use thiserror::Error;
use  std::string::FromUtf8Error;


#[derive(Debug, Error)]
pub enum WallpaperError {
    
    #[error("failed to execute {command}: {source}")]
    Command {
        command: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid UTF-8 output from gsettings: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("failed to scan wallpaper: {0}")]
    Scan(String),

    #[error("invalid wallpaper URI: {0}")]
    InvalidUri(String),

    #[error("gsettings {operation} failed with status {status}")]
    GSettings {
        operation: &'static str,
        status: std::process::ExitStatus,
    },

    #[error("no alternative wallpaper found")]
    NoAlternative
}