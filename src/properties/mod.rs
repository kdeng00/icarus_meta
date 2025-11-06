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
        Ok(flac_properties) => Ok(SongProperties {
            duration: flac_properties.duration(),
            sample_rate: flac_properties.sample_rate(),
            bitrate: flac_properties.audio_bitrate(),
            overall_bitrate: flac_properties.overall_bitrate(),
            bit_depth: flac_properties.bit_depth(),
            channels: flac_properties.channels(),
        }),
        Err(err) => Err(err),
    }
}

pub fn get_duration(song_path: &str) -> Result<std::time::Duration, std::io::Error> {
    match get_song_properties(song_path) {
        Ok(song_properties) => Ok(song_properties.duration),
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

    #[test]
    fn test_song_properties() {
        let filename = test_util::util::get_filename(1);
        let dir = String::from(test_util::util::TESTFILEDIRECTORY);

        match test_util::util::file_exists(&dir, &filename) {
            Ok(_) => {
                let filepath = test_util::util::get_full_path(&dir, &filename).unwrap();
                let new_filepath = test_util::util::generate_newfilepath(&dir).unwrap();

                match test_util::util::copy_file(&filepath, &new_filepath) {
                    Ok(_) => match super::get_song_properties(&filepath) {
                        Ok(song_properties) => {
                            let song_duration: u64 = 41;
                            let bitrate: u32 = 1;
                            let overall_bitrate: u32 = 3;
                            let bit_depth: u8 = 24;
                            let channels: u8 = 2;

                            let fetched_song_duration = song_properties.duration.as_secs();
                            let fetched_bitrate = song_properties.bitrate;
                            let fetched_overall_bitrate = song_properties.overall_bitrate;
                            let fetched_bit_depth = song_properties.bit_depth;
                            let fetched_channels = song_properties.channels;

                            assert_eq!(
                                song_duration, fetched_song_duration,
                                "Durations should match, but they don't {song_duration} {fetched_song_duration} ({song_properties:?})"
                            );

                            assert_eq!(
                                bitrate, fetched_bitrate,
                                "Bitrates do not match {bitrate:?} {fetched_bitrate:?} {song_properties:?}"
                            );
                            assert_eq!(
                                overall_bitrate, fetched_overall_bitrate,
                                "Overall bitrates do not match {overall_bitrate:?} {fetched_overall_bitrate:?} {song_properties:?}"
                            );
                            assert_eq!(
                                bit_depth, fetched_bit_depth,
                                "Bit depth do not match {bit_depth:?} {fetched_bit_depth:?} {song_properties:?}"
                            );
                            assert_eq!(
                                channels, fetched_channels,
                                "Channels do not match {channels:?} {fetched_channels:?} {song_properties:?}"
                            );
                        }
                        Err(err) => {
                            assert!(false, "Error: {err:?}");
                        }
                    },
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
