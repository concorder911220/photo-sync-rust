use std::fs;
use photo_sync::{DayPhotos, sync_photos};

fn main() {
    let data = fs::read_to_string("inputs/test_case_1.json").expect("Unable to read file");
    
    let day_photos: DayPhotos = serde_json::from_str(&data).expect("JSON was not well-formatted");

    let missing_photos = sync_photos(&day_photos);
    for (photo, friend) in missing_photos {
        println!("Missing photo: {}, can be found with: {}", photo, friend);
    }
}
