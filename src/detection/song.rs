pub fn file_type_from_filepath(
    filepath: &str,
) -> Result<crate::detection::FileType, std::io::Error> {
    match infer::get_from_path(filepath) {
        Ok(Some(kind)) => {
            let mime = kind.mime_type();
            if mime == "audio/x-flac" {
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

pub mod constants {
    pub const FLAC_TYPE: &str = "flac";
}
