/// Gets the file type of a CoverArt given it's path
pub fn file_type_from_filepath(
    filepath: &str,
) -> Result<crate::detection::FileType, std::io::Error> {
    match imghdr::from_file(filepath) {
        Ok(Some(imghdr::Type::Jpeg)) => Ok(crate::detection::FileType {
            mime: String::from(constants::mime::JPEG),
            file_type: String::from(constants::JPEG_TYPE),
        }),
        Ok(Some(imghdr::Type::Png)) => Ok(crate::detection::FileType {
            mime: String::from(constants::mime::PNG),
            file_type: String::from(constants::PNG_TYPE),
        }),
        Ok(None) => Err(std::io::Error::other("Image file not supported")),
        Err(err) => Err(err),
        _ => Err(std::io::Error::other("Image file not supported")),
    }
}

/// Gets the file type of a CoverArt given it's data
pub fn file_type_from_data(data: &Vec<u8>) -> Result<crate::detection::FileType, std::io::Error> {
    match imghdr::from_bytes(data) {
        Some(imghdr::Type::Jpeg) => Ok(crate::detection::FileType {
            mime: String::from(constants::mime::JPEG),
            file_type: String::from(constants::JPEG_TYPE),
        }),
        Some(imghdr::Type::Png) => Ok(crate::detection::FileType {
            mime: String::from(constants::mime::PNG),
            file_type: String::from(constants::PNG_TYPE),
        }),
        None => Err(std::io::Error::other("Image file not supported")),
        _ => Err(std::io::Error::other("Image file not supported")),
    }
}

pub mod constants {
    pub const PNG_TYPE: &str = "png";
    pub const JPEG_TYPE: &str = "jpeg";
    pub const JPG_TYPE: &str = "jpg";

    pub mod mime {
        pub const JPEG: &str = "image/jpeg";
        pub const PNG: &str = "image/png";
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coverart_file_type() {
        let directory = String::from(crate::test_util::util::TESTFILEDIRECTORY);
        let filename = String::from("Sample Tracks 3.png");
        let filepath = format!("{directory}/{filename}");

        match super::file_type_from_filepath(&filepath) {
            Ok(filetype) => {
                assert_eq!(
                    filetype.file_type,
                    super::constants::PNG_TYPE,
                    "The file type of the CoverArt should have been png"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }

    #[test]
    fn test_coverart_file_type_from_data() {
        let directory = String::from(crate::test_util::util::TESTFILEDIRECTORY);
        let filename = String::from("Sample Tracks 3.png");
        let filepath = format!("{directory}/{filename}");
        let data = crate::test_util::util::get_data_from_file(&filepath).unwrap();

        match super::file_type_from_data(&data) {
            Ok(filetype) => {
                assert_eq!(
                    filetype.file_type,
                    super::constants::PNG_TYPE,
                    "The file type of the CoverArt should have been png"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
