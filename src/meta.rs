use lofty::{file::AudioFile, tag::Accessor};

use crate::types;

fn get_type(t: types::Type) -> Result<String, std::io::Error> {
    match t {
        types::Type::Title => Ok("TITLE".to_owned()),
        types::Type::Artist => Ok("ARTIST".to_owned()),
        types::Type::Album => Ok("ALBUM".to_owned()),
        types::Type::AlbumArtist => Ok("ALBUMARTIST".to_owned()),
        types::Type::Genre => Ok("GENRE".to_owned()),
        types::Type::Date => Ok("DATE".to_owned()),
        types::Type::Track => Ok("TRACKNUMBER".to_owned()),
        types::Type::Disc => Ok("DISCNUMBER".to_owned()),
    }
}

pub fn get_meta(t: types::Type, filepath: &String) -> Result<String, std::io::Error> {
    match std::fs::File::open(filepath) {
        Ok(mut content) => {
            match lofty::flac::FlacFile::read_from(&mut content, lofty::config::ParseOptions::new())
            {
                Ok(flac_file) => match flac_file.vorbis_comments() {
                    Some(vb) => {
                        let type_str: String = get_type(t).unwrap();
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
            match lofty::flac::FlacFile::read_from(&mut content, lofty::config::ParseOptions::new())
            {
                Ok(mut flac_file) => match flac_file.vorbis_comments_mut() {
                    Some(vb) => {
                        let pre_value = value.clone();
                        match t {
                            types::Type::Album => {
                                vb.set_album(pre_value);
                            }
                            types::Type::AlbumArtist => {
                                vb.insert(get_type(t).unwrap(), pre_value);
                            }
                            types::Type::Artist => {
                                vb.set_artist(pre_value);
                            }
                            types::Type::Date => {
                                vb.insert(get_type(t).unwrap(), pre_value);
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

#[cfg(test)]
mod tests {
    use util::{file_exists, get_full_path};

    use super::*;

    mod util {
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
    }

    mod set {
        use super::*;

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
}
