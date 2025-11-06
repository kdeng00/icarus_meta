use lofty::file::AudioFile;

pub fn get_duration(song_path: &String) -> Result<std::time::Duration, std::io::Error> {
    match std::fs::File::open(song_path) {
        Ok(mut content) => {
            match lofty::flac::FlacFile::read_from(&mut content, lofty::config::ParseOptions::new())
            {
                Ok(flac_file) => {
                    let properties = flac_file.properties();
                    Ok(properties.duration())
                }
                Err(err) => Err(std::io::Error::other(err.to_string())),
            }
        }
        Err(err) => Err(err),
    }
}
