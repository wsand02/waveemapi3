use std::path::Path;

use uuid::Uuid;

const MP3_EXT: &str = ".mp3";
const WAV_EXT: &str = ".wav";

pub fn wav_path(data_path: &String) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, WAV_EXT);
    Path::new(&data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

pub fn mp3_path(data_path: &String) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, MP3_EXT);
    Path::new(&data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const FNAME_LEN: usize = 40; // 36 (uuid) + 4 (.mp3)

    #[test]
    fn test_wav_path_format() {
        let data_path = "/tmp/audio".to_string();
        let result = wav_path(&data_path);

        assert!(result.starts_with("/tmp/audio"));

        assert!(result.ends_with(".wav"));

        let filename = Path::new(&result).file_name().unwrap().to_str().unwrap();
        assert!(filename.len() == FNAME_LEN);
    }

    #[test]
    fn test_mp3_path_format() {
        let data_path = "/tmp/audio".to_string();
        let result = mp3_path(&data_path);

        assert!(result.starts_with("/tmp/audio"));

        assert!(result.ends_with(".mp3"));

        let filename = Path::new(&result).file_name().unwrap().to_str().unwrap();
        assert!(filename.len() == FNAME_LEN);
    }

    #[test]
    fn test_unique_paths() {
        let data_path = "/tmp/audio".to_string();

        // Generate multiple paths and ensure they're unique
        let wav1 = wav_path(&data_path);
        let wav2 = wav_path(&data_path);
        let mp3_1 = mp3_path(&data_path);
        let mp3_2 = mp3_path(&data_path);

        assert_ne!(wav1, wav2);
        assert_ne!(mp3_1, mp3_2);
        assert_ne!(wav1, mp3_1);
    }

    #[test]
    fn test_different_data_paths() {
        let path1 = "/tmp/audio1".to_string();
        let path2 = "/tmp/audio2".to_string();

        let result1 = wav_path(&path1);
        let result2 = wav_path(&path2);

        assert!(result1.starts_with("/tmp/audio1"));
        assert!(result2.starts_with("/tmp/audio2"));
    }

    #[test]
    fn test_empty_data_path() {
        let empty_path = "".to_string();
        let result = wav_path(&empty_path);

        // Should still generate a valid filename with UUID
        assert!(result.ends_with(".wav"));
        let filename = Path::new(&result).file_name().unwrap().to_str().unwrap();
        assert!(filename.len() == FNAME_LEN);
    }

    #[test]
    fn test_relative_path() {
        let relative_path = "./audio".to_string();
        let result = mp3_path(&relative_path);

        assert!(result.starts_with("./audio"));
        assert!(result.ends_with(".mp3"));
    }

    #[test]
    fn test_path_validity() {
        let data_path = "/tmp/test".to_string();
        let wav_result = wav_path(&data_path);
        let mp3_result = mp3_path(&data_path);

        // Test that the generated paths are valid Path objects
        let wav_path_obj = Path::new(&wav_result);
        let mp3_path_obj = Path::new(&mp3_result);

        assert!(wav_path_obj.is_absolute());
        assert!(mp3_path_obj.is_absolute());

        // Test that parent directory is as expected
        assert_eq!(wav_path_obj.parent().unwrap(), Path::new("/tmp/test"));
        assert_eq!(mp3_path_obj.parent().unwrap(), Path::new("/tmp/test"));
    }

    #[test]
    fn test_uuid_format() {
        let data_path = "/tmp".to_string();
        let result = wav_path(&data_path);

        let filename = Path::new(&result).file_stem().unwrap().to_str().unwrap();

        assert!(
            Uuid::parse_str(filename).is_ok(),
            "Generated filename should be a valid UUID"
        );
    }
}
