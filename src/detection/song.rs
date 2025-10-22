/// Gets the file type of a Song from it's path
pub fn file_type_from_filepath(
    filepath: &str,
) -> Result<crate::detection::FileType, std::io::Error> {
    match infer::get_from_path(filepath) {
        Ok(Some(kind)) => {
            let mime = kind.mime_type();
            if mime == constants::mime::FLAC {
                Ok(crate::detection::FileType {
                    mime: String::from(mime),
                    file_type: String::from(constants::FLAC_TYPE),
                })
            } else {
                Err(std::io::Error::other("Unsupported file type"))
            }
        }
        Ok(None) => Err(std::io::Error::other("File type not determined")),
        Err(err) => Err(err),
    }
}

/// Gets the file type of a Song given it's data
pub fn file_type_from_data(data: &Vec<u8>) -> Result<crate::detection::FileType, std::io::Error> {
    match infer::get(data) {
        Some(kind) => {
            let mime = kind.mime_type();
            if mime == constants::mime::FLAC {
                Ok(crate::detection::FileType {
                    mime: String::from(mime),
                    file_type: String::from(constants::FLAC_TYPE),
                })
            } else {
                Err(std::io::Error::other("Unsupported file type"))
            }
        }
        None => Err(std::io::Error::other("File type not determined")),
    }
}

pub mod constants {
    pub const FLAC_TYPE: &str = "flac";

    pub mod mime {
        pub const FLAC: &str = "audio/x-flac";
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_song_file_type() {
        let directory = String::from(crate::test_util::util::TESTFILEDIRECTORY);
        let filename = String::from("track01.flac");
        let filepath = format!("{directory}/{filename}");

        match super::file_type_from_filepath(&filepath) {
            Ok(filetype) => {
                assert_eq!(
                    filetype.file_type,
                    crate::detection::song::constants::FLAC_TYPE,
                    "Types do not match"
                )
            }
            Err(err) => {
                assert!(false, "Error: {err:?}")
            }
        }
    }

    #[test]
    fn test_song_file_type_from_data() {
        let directory = String::from(crate::test_util::util::TESTFILEDIRECTORY);
        let filename = String::from("track01.flac");
        let filepath = format!("{directory}/{filename}");
        let data = crate::test_util::util::get_data_from_file(&filepath).unwrap();

        match super::file_type_from_data(&data) {
            Ok(filetype) => {
                assert_eq!(
                    filetype.file_type,
                    crate::detection::song::constants::FLAC_TYPE,
                    "Types do not match"
                )
            }
            Err(err) => {
                assert!(false, "Error: {err:?}")
            }
        }
    }
}
