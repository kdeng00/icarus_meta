pub mod meta_type {
    pub enum Type {
        Title,
        Artist,
        Album,
        Genre,
        Year,
        Track,
    }
}

pub mod meta_nouveaou {
    use super::*;

    pub fn get_meta(t: meta_type::Type, filepath: &String) -> Result<String, std::io::Error> {
        match t {
            meta_type::Type::Title => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            meta_type::Type::Artist => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            meta_type::Type::Album => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            meta_type::Type::Genre => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            meta_type::Type::Year => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            meta_type::Type::Track => match new_meta(filepath) {
                Ok(metaa) => match get_val(t, metaa.tags) {
                    Ok(val) => Ok(val),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
        }
    }

    fn get_type(t: meta_type::Type) -> Result<String, std::io::Error> {
        match t {
            meta_type::Type::Title => Ok("TITLE".to_string()),
            meta_type::Type::Artist => Ok("ARTIST".to_string()),
            meta_type::Type::Album => Ok("".to_string()),
            meta_type::Type::Genre => Ok("".to_string()),
            meta_type::Type::Year => Ok("".to_string()),
            meta_type::Type::Track => Ok("".to_string()),
        }
    }

    fn get_val(t: meta_type::Type, tags: Vec<(String, String)>) -> Result<String, std::io::Error> {
        let type_ma: String = get_type(t).unwrap();
        for tag in tags {
            if tag.0 == type_ma {
                return Ok(tag.1);
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid",
        ))
    }

    fn new_meta(filepath: &String) -> Result<metadata::MediaFileMetadata, std::io::Error> {
        let path = std::path::Path::new(&filepath);
        metadata::MediaFileMetadata::new(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_title() {
        let filename = String::from("track01.flac");
        let dir = String::from("tests/sample_tracks3");
        let dir_path = std::path::Path::new(&dir);
        let full_path = dir_path.join(filename);

        println!("Path: {:?}", full_path);

        assert!(full_path.exists(), "Path does not exists {:?}", full_path);
        let filepath = full_path.display().to_string();

        match meta_nouveaou::get_meta(meta_type::Type::Title, &filepath) {
            Ok(title) => {
                let found = title == "Just roll it";
                assert!(found, "Meta information was not found {:?}", title);
            }
            Err(err) => {
                assert!(false, "Error: {:?}", err);
            }
        }
    }
}
