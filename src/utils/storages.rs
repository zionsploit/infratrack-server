use std::{fs::{DirBuilder, File}, ops::Add, io::Write, path::Path};


#[allow(unused)]
pub struct Storage;

static PATH: &str = "Storage";

#[allow(unused)]
impl Storage {

    // FOLDER BASED
    pub fn create_storage_if_not_exists () {
        let check_path = Path::new(PATH).is_dir();
        if !check_path {
            DirBuilder::new().create(PATH).unwrap();
        }
    }

    pub fn create_folder (foldername: &str) {
        let new_path = String::from(PATH.to_string().add("/").add(foldername));
        let check_path = Path::new(&new_path).is_dir();

        if !check_path {
            DirBuilder::new().create(new_path).unwrap();
        }
    }

    // FILE BASED
    pub fn create_file (foldername: &str, file_name: &str, file_bytes: &[u8]) -> Option<String> {
        let new_path = String::from(PATH.to_string().add("/").add(foldername));

        let check_path = Path::new(&new_path).is_dir();

        if !check_path {
            return None;
        }

        let mut file = File::create(new_path.add("/").add(file_name)).unwrap();

        file.write_all(file_bytes);

        Some("Done".to_string())
    }
}