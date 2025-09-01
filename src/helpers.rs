use std::path::Path;

use uuid::Uuid;

use std::fs;
use std::io;

const MP3_EXT: &str = ".mp3";
const WAV_EXT: &str = ".wav";
const FNAME_LEN: usize = 40; // 36 (uuid) + 4 (.mp3)

/// Deletes all .wav and .mp3 files in `data_path` that are exactly 40 characters long (including extension).
pub fn clear_data_path(data_path: &str) -> io::Result<()> {
    let dir = Path::new(data_path);
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
            let is_wav = fname.ends_with(WAV_EXT);
            let is_mp3 = fname.ends_with(MP3_EXT);
            if (is_wav || is_mp3) && fname.len() == FNAME_LEN {
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

pub fn wav_path(data_path: &str) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, WAV_EXT);
    Path::new(data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

pub fn mp3_path(data_path: &str) -> String {
    let id = Uuid::new_v4();
    let filename = format!("{}{}", id, MP3_EXT);
    Path::new(data_path)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

#[allow(dead_code)]
fn get_unique_data_path() -> String {
    let unique_id = Uuid::new_v4();
    format!("/tmp/{}", unique_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_wav_path_format() {
        let data_path = get_unique_data_path();
        let result = wav_path(&data_path);

        assert!(result.starts_with(data_path.as_str()));

        assert!(result.ends_with(".wav"));

        let filename = Path::new(&result).file_name().unwrap().to_str().unwrap();
        assert!(filename.len() == FNAME_LEN);
    }

    #[test]
    fn test_mp3_path_format() {
        let data_path = get_unique_data_path();
        let result = mp3_path(&data_path);

        assert!(result.starts_with(data_path.as_str()));

        assert!(result.ends_with(".mp3"));

        let filename = Path::new(&result).file_name().unwrap().to_str().unwrap();
        assert!(filename.len() == FNAME_LEN);
    }

    #[test]
    fn test_unique_paths() {
        let data_path1 = get_unique_data_path();
        let data_path2 = get_unique_data_path();
        let data_path3 = get_unique_data_path();
        let data_path4 = get_unique_data_path();

        // Generate multiple paths and ensure they're unique
        let wav1 = wav_path(&data_path1);
        let wav2 = wav_path(&data_path2);
        let mp3_1 = mp3_path(&data_path3);
        let mp3_2 = mp3_path(&data_path4);

        assert_ne!(wav1, wav2);
        assert_ne!(mp3_1, mp3_2);
        assert_ne!(wav1, mp3_1);
    }

    #[test]
    fn test_different_data_paths() {
        let path1 = get_unique_data_path();
        let path2 = get_unique_data_path();

        let result1 = wav_path(&path1);
        let result2 = wav_path(&path2);

        assert!(result1.starts_with(path1.as_str()));
        assert!(result2.starts_with(path2.as_str()));
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

    #[test]
    fn test_clear_data_path() {
        let tmpdir = tempfile::tempdir().unwrap();
        let test_dir = tmpdir.path();
        println!("Test directory: {:?}", test_dir);

        for _ in 0..5 {
            let wav_file = test_dir.join(format!("{}.wav", Uuid::new_v4()));
            let mp3_file = test_dir.join(format!("{}.mp3", Uuid::new_v4()));
            fs::write(&wav_file, b"test").unwrap();
            fs::write(&mp3_file, b"test").unwrap();
        }
        // Create some non-matching files

        let other_file = test_dir.join("not_to_delete.txt");
        fs::write(&other_file, b"keep me").unwrap();
        clear_data_path(test_dir.to_string_lossy().as_ref()).unwrap();
        assert!(other_file.exists(), "Non-matching file should remain");
        for entry in fs::read_dir(test_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if let Some(fname) = path.file_name().and_then(|n| n.to_str()) {
                let is_wav = fname.ends_with(WAV_EXT);
                let is_mp3 = fname.ends_with(MP3_EXT);
                if (is_wav || is_mp3) && fname.len() == 40 {
                    panic!("File {} should have been deleted", fname);
                }
            }
        }
        fs::remove_file(other_file).ok(); // clean up
    }
}
