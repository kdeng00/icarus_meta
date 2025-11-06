use lofty::file::AudioFile;

pub fn get_duration(song_path: &String) -> Result<std::time::Duration, std::io::Error> {
    match std::fs::File::open(song_path) {
        Ok(mut content) => {
            match lofty::flac::FlacFile::read_from(&mut content, lofty::config::ParseOptions::new())
            {
                Ok(flac_file) => {
                    let properties = flac_file.properties();
                    Ok(properties.duration())
                }
                Err(err) => Err(std::io::Error::other(err.to_string())),
            }
        }
        Err(err) => Err(err),
    }
}

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
                match super::get_duration(&filepath) {
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
