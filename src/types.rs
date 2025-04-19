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
