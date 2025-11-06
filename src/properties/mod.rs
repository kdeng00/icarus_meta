pub mod audio;


#[derive(Clone, Debug, Default)]
pub struct SongProperties {
    pub duration: std::time::Duration,
    pub sample_rate: u32,
    pub bitrate: u32,
    pub overall_bitrate: u32,
    pub bit_depth: u8,
    pub channels: u8,
}


pub fn get_song_properties(song_path: &str) -> Result<SongProperties, std::io::Error> {
    match audio::get_properties(song_path) {
        Ok(flac_properties) => {
            Ok(SongProperties {
                duration: flac_properties.duration(),
                sample_rate: flac_properties.sample_rate(),
                bitrate: flac_properties.audio_bitrate(),
                overall_bitrate: flac_properties.overall_bitrate(),
                bit_depth: flac_properties.bit_depth(),
                channels: flac_properties.channels(),
            })
        }
        Err(err) => Err(err),
    }
}

pub fn get_duration(song_path: &String) -> Result<std::time::Duration, std::io::Error> {
    match get_song_properties(song_path) {
        Ok(song_properties) => {
            Ok(song_properties.duration)
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
