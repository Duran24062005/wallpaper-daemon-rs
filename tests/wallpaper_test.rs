#[path = "../src/error.rs"]
#[allow(dead_code)]
mod error;
#[path = "../src/wallpapers.rs"]
#[allow(dead_code)]
mod wallpapers;

use std::path::Path;

#[test]
fn acepta_uri_de_un_archivo_que_ya_no_existe() {
    let result = wallpapers::parse_wallpaper_uri("'file:///tmp/wallpaper-eliminado.jpg'");

    assert_eq!(result.unwrap(), Path::new("/tmp/wallpaper-eliminado.jpg"));
}

#[test]
fn rechaza_un_valor_que_no_es_una_uri_file() {
    let result = wallpapers::parse_wallpaper_uri("'/tmp/wallpaper.jpg'");

    assert!(result.is_err());
}
