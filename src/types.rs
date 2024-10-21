use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Number(Number),
    String(String),
    Boolean(bool),
    Document(Box<JsonDocument>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonDocument {
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonDocument {
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonDocument::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonDocument::Object(obj) => Some(obj),
            _ => None,
        }
    }
}
