use rand::RngExt;
use std::path::{Path, PathBuf};


pub fn select_random<'a>(images: &'a [PathBuf], current: Option<&Path>) -> Option<&'a PathBuf>{

    let available: Vec<&PathBuf> = images
        .iter()
        .filter(|image| {
            current
                .map(|current| image.as_path() != current)
                .unwrap_or(true)
        }).collect();

    if available.is_empty() {
        return None;
    }

    let mut rng = rand::rng();
    let index = rng.random_range(0..available.len());

    Some(available[index])

}