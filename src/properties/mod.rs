pub mod properties;

#[cfg(test)]
mod tests {
    use crate::test_util;

    #[test]
    fn test_get_duration() {
        let filename = test_util::util::get_filename(1);
        let dir = String::from(test_util::util::TESTFILEDIRECTORY);

        match test_util::util::file_exists(&dir, &filename) {
            Ok(_) => {
                let filepath = test_util::util::get_full_path(&dir, &filename).unwrap();
                match super::properties::get_duration(&filepath) {
                    Ok(duration) => {
                        let song_duration: u64 = 41;
                        let fetched_song_duration = duration.as_secs();

                        assert_eq!(
                            song_duration, fetched_song_duration,
                            "Durations should match, but they don't {song_duration} {fetched_song_duration} ({duration:?})"
                        );
                    }
                    Err(err) => {
                        assert!(false, "Error: {err:?}");
                    }
                }
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
