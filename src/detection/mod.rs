pub mod coverart;
pub mod song;

#[derive(Debug, Default)]
pub struct FileType {
    pub mime: String,
    pub file_type: String,
}
