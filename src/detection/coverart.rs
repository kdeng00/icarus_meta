/// Gets the file type of a CoverArt given it's path
pub fn file_type(filepath: &str) -> Result<String, std::io::Error> {
    match imghdr::from_file(filepath) {
        Ok(Some(imghdr::Type::Jpeg)) => Ok(String::from("jpeg")),
        Ok(Some(imghdr::Type::Png)) => Ok(String::from("png")),
        Ok(None) => Err(std::io::Error::other("Image file not supported")),
        Err(err) => Err(err),
        _ => Err(std::io::Error::other("Image file not supported")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_coverart_file_type() {
        let directory = String::from(crate::test_util::util::TESTFILEDIRECTORY);
        let filename = String::from("Sample Tracks 3.png");
        let filepath = format!("{directory}/{filename}");

        match super::file_type(&filepath) {
            Ok(filetype) => {
                assert_eq!(
                    filetype, "png",
                    "The file type of the CoverArt should have been png"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
