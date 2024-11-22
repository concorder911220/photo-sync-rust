use photo_sync::{DayPhotos, sync_photos};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_photos() {
        let day_photos = DayPhotos {
            date: "2024-04-15".to_string(),
            my_photos: vec!["photo1".to_string(), "photo2".to_string()].into_iter().collect(),
            friend1_photos: vec!["photo2".to_string(), "photo3".to_string()].into_iter().collect(),
            friend2_photos: vec!["photo4".to_string(), "photo5".to_string()].into_iter().collect(),
        };

        let missing_photos = sync_photos(&day_photos);

        assert_eq!(missing_photos.len(), 3);
        assert!(missing_photos.contains(&(String::from("photo3"), String::from("Friend 1"))));
        assert!(missing_photos.contains(&(String::from("photo4"), String::from("Friend 2"))));
        assert!(missing_photos.contains(&(String::from("photo5"), String::from("Friend 2"))));
    }

    #[test]
    fn test_sync_photos_no_missing() {
        let day_photos = DayPhotos {
            date: "2024-04-15".to_string(),
            my_photos: vec!["photo1".to_string(), "photo2".to_string(), "photo3".to_string()].into_iter().collect(),
            friend1_photos: vec!["photo2".to_string(), "photo3".to_string()].into_iter().collect(),
            friend2_photos: vec!["photo4".to_string(), "photo5".to_string()].into_iter().collect(),
        };

        let missing_photos = sync_photos(&day_photos);

        assert_eq!(missing_photos.len(), 2);
        assert!(missing_photos.contains(&(String::from("photo4"), String::from("Friend 2"))));
        assert!(missing_photos.contains(&(String::from("photo5"), String::from("Friend 2"))));
    }
}
