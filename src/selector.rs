use rand::RngExt;
use std::path::PathBuf;


pub fn select_random(images: &[PathBuf]) -> Option<&PathBuf>{
    if images.is_empty() {
        return None;
    }

    let mut rng = rand::rng();
    let index = rng.random_range(0..images.len());

    Some(&images[index])

}