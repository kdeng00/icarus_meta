#[derive(Debug)]
pub enum Type {
    Title,
    Artist,
    Album,
    AlbumArtist,
    Genre,
    Date,
    Track,
    Disc,
    TrackCount,
    DiscCount,
}

#[derive(Debug)]
pub enum MetadataType {
    String(String),
    Int(i32),
}

impl MetadataType {
    pub fn from_std_str(s: &str) -> Self {
        MetadataType::String(s.to_string())
    }

    pub fn from_string(s: String) -> Self {
        MetadataType::String(s)
    }

    pub fn from_int(i: i32) -> Self {
        MetadataType::Int(i)
    }
}

pub mod access {
    pub fn get_type(t: super::Type) -> Result<String, std::io::Error> {
        match t {
            super::Type::Title => Ok("TITLE".to_owned()),
            super::Type::Artist => Ok("ARTIST".to_owned()),
            super::Type::Album => Ok("ALBUM".to_owned()),
            super::Type::AlbumArtist => Ok("ALBUMARTIST".to_owned()),
            super::Type::Genre => Ok("GENRE".to_owned()),
            super::Type::Date => Ok("DATE".to_owned()),
            super::Type::Track => Ok("TRACKNUMBER".to_owned()),
            super::Type::Disc => Ok("DISCNUMBER".to_owned()),
            super::Type::TrackCount => Ok("TRACKCOUNT".to_owned()),
            super::Type::DiscCount => Ok("DISCCOUNT".to_owned()),
        }
    }
}
