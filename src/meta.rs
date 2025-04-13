use lofty::file::AudioFile;

use crate::types;

fn get_type(t: types::Type) -> Result<String, std::io::Error> {
    match t {
        types::Type::Title => Ok("TITLE".to_owned()),
        types::Type::Artist => Ok("ARTIST".to_owned()),
        types::Type::Album => Ok("ALBUM".to_owned()),
        types::Type::Genre => Ok("GENRE".to_owned()),
        types::Type::Date => Ok("DATE".to_owned()),
        types::Type::Track => Ok("TRACKNUMBER".to_owned()),
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

        pub fn test_file_directory() -> String {
            String::from("tests/sample_tracks3")
        }
    }

    #[test]
    fn test_get_title() {
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

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
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

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
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

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
    fn test_get_genre() {
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

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
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

        match file_exists(&dir, &filename) {
            Ok(_) => {
                let filepath = get_full_path(&dir, &filename).unwrap();

                match get_meta(types::Type::Date, &filepath) {
                    Ok(year) => {
                        let found = year == "2025-04-11";
                        assert!(found, "Meta information was not found {:?}", year);
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
        let filename = String::from("track01.flac");
        let dir = util::test_file_directory();

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
}
