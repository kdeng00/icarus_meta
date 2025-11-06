use lofty::file::AudioFile;

pub fn get_properties(songpath: &str) -> Result<lofty::flac::FlacProperties, std::io::Error> {
    match std::fs::File::open(songpath) {
        Ok(mut content) => {
            match lofty::flac::FlacFile::read_from(&mut content, lofty::config::ParseOptions::new())
            {
                Ok(flac_file) => Ok(*flac_file.properties()),
                Err(err) => Err(std::io::Error::other(err.to_string())),
            }
        }
        Err(err) => Err(err),
    }
}
