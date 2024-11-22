use std::collections::HashSet;
use sha2::{Sha256, Digest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DayPhotos {
    pub date: String,
    pub my_photos: HashSet<String>,
    pub friend1_photos: HashSet<String>,
    pub friend2_photos: HashSet<String>,
}

pub fn calculate_photo_hash(photo: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(photo);
    format!("{:x}", hasher.finalize())
}

pub fn calculate_total_hash(photos: &HashSet<String>) -> String {
    let mut sorted_photos: Vec<&String> = photos.iter().collect();
    sorted_photos.sort();

    let mut hasher = Sha256::new();
    for photo in sorted_photos {
        hasher.update(calculate_photo_hash(photo));
    }
    format!("{:x}", hasher.finalize())
}

pub fn sync_photos(day_photos: &DayPhotos) -> Vec<(String, String)> {
    let mut missing_photos = Vec::new();

    let my_total_hash = calculate_total_hash(&day_photos.my_photos);
    let friend1_total_hash = calculate_total_hash(&day_photos.friend1_photos);
    let friend2_total_hash = calculate_total_hash(&day_photos.friend2_photos);

    if my_total_hash != friend1_total_hash {
        for photo in &day_photos.friend1_photos {
            let _photo_hash = calculate_photo_hash(photo);
            if !day_photos.my_photos.contains(photo) {
                missing_photos.push((photo.clone(), "Friend 1".to_string()));
            }
        }
    }

    if my_total_hash != friend2_total_hash {
        for photo in &day_photos.friend2_photos {
            let _photo_hash = calculate_photo_hash(photo);
            if !day_photos.my_photos.contains(photo) {
                missing_photos.push((photo.clone(), "Friend 2".to_string()));
            }
        }
    }

    missing_photos
}
