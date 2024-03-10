use std::collections::HashMap;

use serde_json::{Map, Value};

pub mod classification;
pub mod funcaptcha;

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
