pub mod coverart {

    use lofty::{file::AudioFile, ogg::OggPictureStorage};

    pub fn set_coverart(
        song_filepath: &String,
        coverart_filepath: &String,
    ) -> Result<Vec<u8>, std::io::Error> {
        let coverart_path = std::path::Path::new(coverart_filepath);

        match std::fs::File::open(song_filepath) {
            Ok(mut file) => {
                match lofty::flac::FlacFile::read_from(
                    &mut file,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(mut flac_file) => {
                        let mut coverart_file = std::fs::File::open(coverart_path).unwrap();
                        match lofty::picture::Picture::from_reader(&mut coverart_file) {
                            Ok(pic) => {
                                match lofty::picture::PictureInformation::from_picture(&pic) {
                                    Ok(info) => {
                                        flac_file.set_picture(0, pic.clone(), info);
                                        Ok(pic.into_data())
                                    }
                                    Err(err) => Err(std::io::Error::new(
                                        std::io::ErrorKind::InvalidData,
                                        err.to_string(),
                                    )),
                                }
                            }
                            Err(err) => Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                err.to_string(),
                            )),
                        }
                    }
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_coverart(song_filepath: &String) -> Result<Vec<u8>, std::io::Error> {
        match std::fs::File::open(song_filepath) {
            Ok(mut file) => {
                match lofty::flac::FlacFile::read_from(
                    &mut file,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(flac_file) => {
                        let pictures = flac_file.pictures();
                        let res = pictures.to_vec();
                        if !res.is_empty() {
                            let picture = &res[0];
                            Ok(picture.clone().0.into_data())
                        } else {
                            Ok(Vec::new())
                        }
                    }
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn contains_coverart(song_filepath: &String) -> Result<(bool, usize), std::io::Error> {
        match std::fs::File::open(song_filepath) {
            Ok(mut file) => {
                match lofty::flac::FlacFile::read_from(
                    &mut file,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(flac_file) => {
                        let pictures = flac_file.pictures();
                        if pictures.is_empty() {
                            Ok((false, 0))
                        } else {
                            let res = pictures.to_vec();
                            if res.is_empty() {
                                Ok((false, 0))
                            } else {
                                Ok((true, res.len()))
                            }
                        }
                    }
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn remove_coverart(song_filepath: &String) -> Result<Vec<u8>, std::io::Error> {
        match std::fs::File::open(song_filepath) {
            Ok(mut file) => {
                match lofty::flac::FlacFile::read_from(
                    &mut file,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(mut flac_file) => {
                        let pictures = flac_file.pictures();
                        let res = pictures.to_vec();
                        if !res.is_empty() {
                            let picture = &res[0];
                            flac_file.remove_picture(0);
                            Ok(picture.clone().0.into_data())
                        } else {
                            Err(std::io::Error::new(
                                std::io::ErrorKind::NotFound,
                                "No pictures found",
                            ))
                        }
                    }
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }
}

pub mod metadata {
    use crate::types;
    use lofty::file::AudioFile;
    use lofty::tag::Accessor;

    pub fn get_meta(t: types::Type, filepath: &String) -> Result<String, std::io::Error> {
        match std::fs::File::open(filepath) {
            Ok(mut content) => {
                match lofty::flac::FlacFile::read_from(
                    &mut content,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(flac_file) => match flac_file.vorbis_comments() {
                        Some(vb) => {
                            let type_str: String = types::access::get_type(t).unwrap();
                            match vb.get(&type_str) {
                                Some(val) => Ok(val.to_owned()),
                                None => Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "Could not get tag data",
                                )),
                            }
                        }
                        None => Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "No tags found",
                        )),
                    },
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn set_meta(
        t: types::Type,
        filepath: &String,
        value: &String,
    ) -> Result<String, std::io::Error> {
        match std::fs::File::open(filepath) {
            Ok(mut content) => {
                match lofty::flac::FlacFile::read_from(
                    &mut content,
                    lofty::config::ParseOptions::new(),
                ) {
                    Ok(mut flac_file) => match flac_file.vorbis_comments_mut() {
                        Some(vb) => {
                            let pre_value = value.clone();
                            match t {
                                types::Type::Album => {
                                    vb.set_album(pre_value);
                                }
                                types::Type::AlbumArtist => {
                                    vb.insert(types::access::get_type(t).unwrap(), pre_value);
                                }
                                types::Type::Artist => {
                                    vb.set_artist(pre_value);
                                }
                                types::Type::Date => {
                                    vb.insert(types::access::get_type(t).unwrap(), pre_value);
                                }
                                types::Type::Disc => {
                                    vb.set_disk(pre_value.parse().unwrap());
                                }
                                types::Type::Genre => {
                                    vb.set_genre(pre_value);
                                }
                                types::Type::Title => {
                                    vb.set_title(pre_value);
                                }
                                types::Type::Track => {
                                    vb.set_track(pre_value.parse().unwrap());
                                }
                                types::Type::TrackCount => {
                                    vb.set_track_total(pre_value.parse().unwrap());
                                }
                                types::Type::DiscCount => {
                                    vb.set_disk_total(pre_value.parse().unwrap());
                                }
                            };

                            Ok(value.to_owned())
                        }
                        None => Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "No tags found",
                        )),
                    },
                    Err(err) => Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use util::{file_exists, get_full_path};

    use super::*;

    mod util {

        use std::io::{self, Write};

        // Function to save a Vec<u8> to a file
        pub fn save_bytes_to_file(bytes: &[u8], file_path: &String) -> io::Result<()> {
            let file = std::path::Path::new(file_path);
            let mut file = std::fs::File::create(file)?;

            match file.write_all(bytes) {
                Ok(_res) => Ok(()),
                Err(err) => Err(err),
            }
        }
        pub fn get_full_path(
            directory: &String,
            filename: &String,
        ) -> Result<String, std::io::Error> {
            match path_buf(directory, filename) {
                Ok(pf) => Ok(pf.display().to_string()),
                Err(err) => Err(err),
            }
        }

        pub fn copy_file(
            source_path: &String,
            destination_path: &String,
        ) -> Result<u64, std::io::Error> {
            let src_path = std::path::Path::new(source_path);
            let dest_path = std::path::Path::new(destination_path);

            match std::fs::copy(src_path, dest_path) {
                Ok(bytes) => Ok(bytes),
                Err(err) => Err(err),
            }
        }

        pub fn file_exists(directory: &String, filename: &String) -> Result<bool, std::io::Error> {
            match path_buf(directory, filename) {
                Ok(pf) => Ok(pf.exists()),
                Err(err) => Err(err),
            }
        }

        fn path_buf(
            directory: &String,
            filename: &String,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let dir_path = std::path::Path::new(&directory);
            Ok(dir_path.join(filename))
        }

        pub const TESTFILEDIRECTORY: &str = "tests/sample_tracks3";

        pub fn get_filename(track: i32) -> String {
            let mut filename = String::from("track");

            if track < 10 {
                filename += "0";
                filename += &track.to_string();
            } else {
                filename += &track.to_string();
            }

            filename += ".flac";

            filename
        }
    }

    mod get {
        use super::metadata::get_meta;
        use super::*;
        use crate::types;

        #[test]
        fn test_get_title() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Title, &filepath) {
                        Ok(title) => {
                            let found = title == "Just roll it";
                            assert!(found, "Meta information was not found {:?}", title);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_artist() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Artist, &filepath) {
                        Ok(artist) => {
                            let found = artist == "KD";
                            assert!(found, "Meta information was not found {:?}", artist);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_album() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Album, &filepath) {
                        Ok(album) => {
                            let found = album == "Sample Tracks 3";
                            assert!(found, "Meta information was not found {:?}", album);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_album_artist() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::AlbumArtist, &filepath) {
                        Ok(album_artist) => {
                            let found = album_artist == "KD";
                            assert!(found, "Meta information was not found {:?}", album_artist);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_genre() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Genre, &filepath) {
                        Ok(genre) => {
                            let found = genre == "Metal";
                            assert!(found, "Meta information was not found {:?}", genre);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
        #[test]
        fn test_get_year() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Date, &filepath) {
                        Ok(date) => {
                            let found = date == "2025-04-11";
                            assert!(found, "Meta information was not found {:?}", date);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_track() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Track, &filepath) {
                        Ok(track) => {
                            let found = track == "1";
                            assert!(found, "Meta information was not found {:?}", track);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_disc() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::Disc, &filepath) {
                        Ok(disc) => {
                            let found = disc == "1";
                            assert!(found, "Meta information was not found {:?}", disc);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_track_total() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::TrackCount, &filepath) {
                        Ok(track_total) => {
                            let found = track_total == "3";
                            assert!(found, "Meta information was not found {:?}", track_total);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_get_disc_total() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match get_meta(types::Type::DiscCount, &filepath) {
                        Ok(disc_total) => {
                            let found = disc_total == "1";
                            assert!(found, "Meta information was not found {:?}", disc_total);
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
    }

    mod set {
        use super::metadata::{get_meta, set_meta};
        use super::*;
        use crate::types;

        #[test]
        fn test_set_title() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Title, &filepath) {
                            Ok(title) => {
                                let found = title == "Just roll it";
                                assert!(found, "Meta information was not found {:?}", title);
                                let new_title = String::from("The wind burned her");

                                match set_meta(types::Type::Title, &new_filepath, &new_title) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_title, m,
                                            "New title does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_artist() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Artist, &filepath) {
                            Ok(artist) => {
                                let found = artist == "KD";
                                assert!(found, "Meta information was not found {:?}", artist);
                                let new_artist = String::from("Pilot");

                                match set_meta(types::Type::Artist, &new_filepath, &new_artist) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_artist, m,
                                            "New artist does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_album() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Album, &filepath) {
                            Ok(title) => {
                                let found = title == "Sample Tracks 3";
                                assert!(found, "Meta information was not found {:?}", title);
                                let new_album = String::from("Sample Tracks 3 Remastered");

                                match set_meta(types::Type::Album, &new_filepath, &new_album) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_album, m,
                                            "New album does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
        #[test]
        fn test_set_album_artist() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::AlbumArtist, &filepath) {
                            Ok(album_artist) => {
                                let found = album_artist == "KD";
                                assert!(found, "Meta information was not found {:?}", album_artist);
                                let new_album_artist = String::from("Bob");

                                match set_meta(
                                    types::Type::AlbumArtist,
                                    &new_filepath,
                                    &new_album_artist,
                                ) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_album_artist, m,
                                            "New album artist does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
        #[test]
        fn test_set_date() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Date, &filepath) {
                            Ok(date) => {
                                let found = date == "2025-04-11";
                                assert!(found, "Meta information was not found {:?}", date);
                                let new_date = String::from("2025-02-01");

                                match set_meta(types::Type::Date, &new_filepath, &new_date) {
                                    Ok(m) => {
                                        assert_eq!(new_date, m, "New date does not match {:?}", m);
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
        #[test]
        fn test_set_track() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Track, &filepath) {
                            Ok(track) => {
                                let found = track == "1";
                                assert!(found, "Meta information was not found {:?}", track);
                                let new_track = String::from("5");

                                match set_meta(types::Type::Track, &new_filepath, &new_track) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_track, m,
                                            "New track does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
        #[test]
        fn test_set_disc() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Disc, &filepath) {
                            Ok(disc) => {
                                let found = disc == "1";
                                assert!(found, "Meta information was not found {:?}", disc);
                                let new_disc = String::from("2");

                                match set_meta(types::Type::Disc, &new_filepath, &new_disc) {
                                    Ok(m) => {
                                        assert_eq!(new_disc, m, "New disc does not match {:?}", m);
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_track_total() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::TrackCount, &filepath) {
                            Ok(track_total) => {
                                let found = track_total == "3";
                                assert!(found, "Meta information was not found {:?}", track_total);
                                let new_track_total = String::from("5");

                                match set_meta(
                                    types::Type::TrackCount,
                                    &new_filepath,
                                    &new_track_total,
                                ) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_track_total, m,
                                            "New track does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_disc_total() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::DiscCount, &filepath) {
                            Ok(disc_total) => {
                                let found = disc_total == "1";
                                assert!(found, "Meta information was not found {:?}", disc_total);
                                let new_disc_total = String::from("2");

                                match set_meta(
                                    types::Type::DiscCount,
                                    &new_filepath,
                                    &new_disc_total,
                                ) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_disc_total, m,
                                            "New disc_total does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_genre() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track08.flac");
            let new_filepath = test_dir + "/" + &test_filename;

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match get_meta(types::Type::Genre, &filepath) {
                            Ok(genre) => {
                                let found = genre == "Metal";
                                assert!(found, "Meta information was not found {:?}", genre);
                                let new_genre = String::from("Blues");

                                match set_meta(types::Type::Genre, &new_filepath, &new_genre) {
                                    Ok(m) => {
                                        assert_eq!(
                                            new_genre, m,
                                            "New genre does not match {:?}",
                                            m
                                        );
                                    }
                                    Err(err) => {
                                        assert!(false, "Error: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(
                                false,
                                "Error: {:?} source {:?} destination {:?}",
                                err, filepath, new_filepath
                            );
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
    }

    mod pictures {

        use super::*;

        #[test]
        fn test_get_picture() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match coverart::get_coverart(&filepath) {
                        Ok(coverart) => {
                            let is_empty = coverart.is_empty();
                            assert_eq!(is_empty, false, "Should not be empty");

                            let mut new_coverart_path: String = test_dir.clone();
                            new_coverart_path += &String::from("/newcovvv.png");
                            let _ = util::save_bytes_to_file(&coverart, &new_coverart_path);
                            let created_file = std::path::Path::new(&new_coverart_path);
                            assert!(
                                created_file.exists(),
                                "Error: {:?} has not been created",
                                new_coverart_path
                            );
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err.to_string());
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_set_picture() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let new_coverart = String::from("Sample Tracks 3 - Other one.png");
            let new_cover_art_path = get_full_path(&dir, &new_coverart).unwrap();

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match coverart::set_coverart(&filepath, &new_cover_art_path) {
                        Ok(bytes) => {
                            assert_eq!(false, bytes.is_empty(), "This should not be empty");
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_picture_exists() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match coverart::contains_coverart(&filepath) {
                        Ok((exists, pictures)) => {
                            assert!(exists, "File should have a cover art");
                            assert!((pictures > 0), "No cover art was found in the file");
                        }
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }

        #[test]
        fn test_remove_picture() {
            let filename = util::get_filename(1);
            let dir = String::from(util::TESTFILEDIRECTORY);

            let temp_file = tempfile::tempdir().expect("Could not create test directory");
            let test_dir = String::from(temp_file.path().to_str().unwrap());
            let test_filename = String::from("track09.flac");
            let new_filepath = get_full_path(&test_dir, &test_filename).unwrap();

            match file_exists(&dir, &filename) {
                Ok(_) => {
                    let filepath = get_full_path(&dir, &filename).unwrap();

                    match util::copy_file(&filepath, &new_filepath) {
                        Ok(_o) => match coverart::remove_coverart(&new_filepath) {
                            Ok(bytes) => {
                                assert_eq!(false, bytes.is_empty(), "This should not be empty");
                            }
                            Err(err) => {
                                assert!(false, "Error: {:?}", err);
                            }
                        },
                        Err(err) => {
                            assert!(false, "Error: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: File does not exist {:?}", err.to_string());
                }
            };
        }
    }
}
