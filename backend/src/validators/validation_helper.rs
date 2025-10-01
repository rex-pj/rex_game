use std::collections::HashMap;
use validator::ValidationErrors;

pub struct ValidationHelper {}

impl ValidationHelper {
    pub fn new() -> Self {
        ValidationHelper {}
    }

    pub fn flatten_errors(&self, validation_error: ValidationErrors) -> HashMap<String, String> {
        let mut errors_hashmap = HashMap::new();

        for (field, errors) in validation_error.field_errors().iter() {
            let message = match errors.get(0) {
                Some(err) => err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid {}", field)),
                None => format!("Invalid {}", field),
            };

            errors_hashmap.insert(field.to_string(), message);
        }
        errors_hashmap
    }
}
