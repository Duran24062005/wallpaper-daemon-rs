#[path = "../src/scanner.rs"]
mod scanner;

use std::fs::{self, File};
use std::path::PathBuf;

fn temporary_directory() -> PathBuf {
    let directory =
        std::env::temp_dir().join(format!("wallpaper-rs-scanner-test-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    directory
}

#[test]
fn escanea_solo_extensiones_de_imagen() {
    let directory = temporary_directory();
    File::create(directory.join("one.jpeg")).unwrap();
    File::create(directory.join("two.PNG")).unwrap();
    File::create(directory.join("notes.txt")).unwrap();

    let images = scanner::scan_images(directory.to_str().unwrap()).unwrap();

    assert_eq!(images.len(), 2);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn devuelve_error_si_el_directorio_no_existe() {
    let directory =
        std::env::temp_dir().join(format!("wallpaper-rs-missing-{}", std::process::id()));

    assert!(scanner::scan_images(directory.to_str().unwrap()).is_err());
}
