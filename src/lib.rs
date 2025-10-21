pub mod detection;
pub mod meta;
pub mod properties;
pub mod types;

pub mod test_util {
    pub mod util {
        use std::io::{self, Read, Write};

        use rand::Rng;

        // Function to save a Vec<u8> to a file
        pub fn save_bytes_to_file(bytes: &[u8], file_path: &String) -> io::Result<()> {
            let file = std::path::Path::new(file_path);
            let mut file = std::fs::File::create(file)?;

            match file.write_all(bytes) {
                Ok(_res) => Ok(()),
                Err(err) => Err(err),
            }
        }
        pub fn get_full_path(
            directory: &String,
            filename: &String,
        ) -> Result<String, std::io::Error> {
            match path_buf(directory, filename) {
                Ok(pf) => Ok(pf.display().to_string()),
                Err(err) => Err(err),
            }
        }

        pub fn copy_file(
            source_path: &String,
            destination_path: &String,
        ) -> Result<u64, std::io::Error> {
            let src_path = std::path::Path::new(source_path);
            let dest_path = std::path::Path::new(destination_path);

            std::fs::copy(src_path, dest_path)
        }

        pub fn get_data_from_file(source_path: &str) -> Result<Vec<u8>, std::io::Error> {
            match std::fs::File::open(source_path) {
                Ok(mut file) => {
                    let mut data: Vec<u8> = Vec::new();
                    match file.read_to_end(&mut data) {
                        Ok(_) => Ok(data),
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            }
        }

        pub fn file_exists(directory: &String, filename: &String) -> Result<bool, std::io::Error> {
            match path_buf(directory, filename) {
                Ok(pf) => Ok(pf.exists()),
                Err(err) => Err(err),
            }
        }

        pub fn generate_filename() -> Result<String, std::io::Error> {
            let mut filename = String::from("track-");
            let length = 20;
            let characters = "abcdef0123456789";
            let amount_of_characters = characters.len() - 1;
            let mut rng = rand::rng();

            for _ in 0..length {
                let index = rng.random_range(0..=amount_of_characters);
                let random_c = characters.chars().nth(index);

                if let Some(c) = random_c {
                    filename.push(c);
                }
            }

            Ok(format!("{filename}.flac"))
        }

        fn path_buf(
            directory: &String,
            filename: &String,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let dir_path = std::path::Path::new(&directory);
            Ok(dir_path.join(filename))
        }

        pub const TESTFILEDIRECTORY: &str = "tests/sample_tracks3";

        pub fn get_filename(track: i32) -> String {
            const FLAC_EXTENSION: &str = ".flac";

            if track < 10 {
                format!("track0{track}{FLAC_EXTENSION}")
            } else {
                format!("track{track}{FLAC_EXTENSION}")
            }
        }
    }
}
