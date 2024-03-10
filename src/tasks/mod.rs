use std::collections::HashMap;

use serde_json::{Map, Value};

/// This module contains all tasks used for classification
pub mod classification;
/// This module contains all tasks used for solving captchas and retrieving a token
pub mod token;

/// Trait used for implementing tasks
pub trait Task {
    fn task_type(&self) -> &'static str;
    fn properties(&self) -> HashMap<String, String>;
    fn serialize(&self) -> Value {
        let mut data = Map::new();

        data.insert(
            "type".to_string(),
            Value::String(self.task_type().to_string()),
        );

        for (property, value) in self.properties() {
            data.insert(property, Value::String(value));
        }

        Value::Object(data)
    }
}
