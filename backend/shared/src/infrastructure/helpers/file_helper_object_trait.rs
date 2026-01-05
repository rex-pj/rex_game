use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

pub trait FileHelperObjectTrait {
    fn get_object<T: for<'a> Deserialize<'a>>(&self, path: &str) -> Option<T>;
    fn save_object<T: Serialize>(&self, path: &str, object: &T) -> Result<(), ErrorKind>;
}
