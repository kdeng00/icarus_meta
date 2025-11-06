pub mod coverart;
pub mod song;

#[derive(Debug, Default)]
pub struct FileType {
    pub mime: String,
    pub file_type: String,
}


/// Initializes a FileType given a filetype passed.
pub fn init_filetype(filetype: &str) -> Result<FileType, std::io::Error> {
    if filetype == song::constants::FLAC_TYPE {
        Ok(FileType {
            mime: String::from(song::constants::mime::FLAC),
            file_type: String::from(song::constants::FLAC_TYPE),
        })
    } else if filetype == coverart::constants::PNG_TYPE {
        Ok(FileType {
            mime: String::from(coverart::constants::mime::PNG),
            file_type: String::from(coverart::constants::PNG_TYPE),
        })
    } else if filetype == coverart::constants::JPEG_TYPE || filetype == coverart::constants::JPG_TYPE {
        Ok(FileType {
            mime: String::from(coverart::constants::mime::JPEG),
            file_type: String::from(coverart::constants::JPEG_TYPE),
        })
    } else {
        Err(std::io::Error::other(format!("Unsupported FileType: {filetype:?}")))
    }
}
