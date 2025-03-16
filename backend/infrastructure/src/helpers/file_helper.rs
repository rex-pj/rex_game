use super::file_helper_object_trait::FileHelperObjectTrait;
use rex_game_domain::helpers::file_helper_trait::FileHelperTrait;
use serde::{Deserialize, Serialize};
use std::{fs, io::ErrorKind, path::Path};

#[derive(Clone)]
pub struct FileHelper {}

impl FileHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileHelperObjectTrait for FileHelper {
    fn get_object<T: for<'a> Deserialize<'a>>(&self, path: &str) -> Option<T> {
        match fs::read_to_string(path) {
            Ok(data) => {
                let p: T = serde_json::from_str(&data).ok()?;
                Some(p)
            }
            Err(_) => None,
        }
    }

    fn save_object<T: Serialize>(&self, path: &str, object: &T) -> Result<(), ErrorKind> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                fs::create_dir(parent).map_err(|_| ErrorKind::Other)?;
            }
        }
        match serde_json::to_string(object) {
            Ok(value) => {
                let file = fs::write(path, value);
                match file {
                    Ok(data) => Ok(data),
                    Err(_) => Err(ErrorKind::Interrupted),
                }
            }
            Err(_) => Err(ErrorKind::InvalidData),
        }
    }
}

impl FileHelperTrait for FileHelper {
    fn get_string(&self, path: &str) -> Option<String> {
        let file = fs::read_to_string(path);
        match file {
            Ok(data) => Some(data),
            Err(_) => None,
        }
    }

    fn save_string(&self, path: &str, value: &str) -> Result<(), ErrorKind> {
        let file = fs::write(path, value);
        match file {
            Ok(data) => Ok(data),
            Err(_) => Err(ErrorKind::Interrupted),
        }
    }

    fn has_file(&self, path: &str) -> bool {
        if path.is_empty() {
            return false;
        }
        match fs::exists(path) {
            Ok(is_exist) => is_exist,
            Err(_) => false,
        }
    }
}
