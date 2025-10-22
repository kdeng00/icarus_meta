

pub fn file_type_from_filepath(filepath: &str) -> Result<String, std::io::Error> {
    match infer::get_from_path(filepath) {
        Ok(Some(kind)) => {
            Ok(String::new())
        }
        Ok(None) => {
            Err(std::io::Error::other("File type not determined"))
        }
        Err(err) => {
            Err(err)
        }
    }
}

pub mod constants {
    pub const FLAC_TYPE: &str = "flac";
}
