# Nail JSON Parser

A lightweight, fast JSON parser written in Rust that provides robust parsing capabilities with comprehensive error handling.

## Features

- **Complete JSON support**: Parses all JSON data types (objects, arrays, strings, numbers, booleans, null)
- **Escape sequence handling**: Full support for JSON string escape sequences including unicode (`\uXXXX`)
- **Number parsing**: Handles both integers and floating-point numbers with proper validation
- **Error handling**: Detailed error messages for malformed JSON
- **Zero dependencies**: Pure Rust implementation with no external dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nail = { path = "." }
```

## Usage

### Basic Usage

```rust
use nail::{parse_json, JsonDocument, JsonValue};

fn main() {
    let json_string = r#"{"name": "John", "age": 30, "active": true}"#;
    
    match parse_json(json_string) {
        Ok(document) => {
            println!("Parsed successfully: {:?}", document);
            
            // Access as object
            if let Some(object) = document.as_object() {
                if let Some(JsonValue::String(name)) = object.get("name") {
                    println!("Name: {}", name);
                }
            }
        }
        Err(error) => {
            eprintln!("Parse error: {}", error);
        }
    }
}
```

### Parsing Arrays

```rust
use nail::parse_json;

let json_array = r#"[1, 2.5, "hello", true, null]"#;
match parse_json(json_array) {
    Ok(document) => {
        if let Some(array) = document.as_array() {
            println!("Array length: {}", array.len());
            for (i, value) in array.iter().enumerate() {
                println!("  [{}]: {:?}", i, value);
            }
        }
    }
    Err(error) => eprintln!("Error: {}", error),
}
```

### Handling Escape Sequences

```rust
use nail::parse_json;

let json_with_escapes = r#"{"message": "Hello \"World\"\nNew line", "path": "C:\\Users\\test"}"#;
match parse_json(json_with_escapes) {
    Ok(document) => println!("Parsed: {:?}", document),
    Err(error) => eprintln!("Error: {}", error),
}
```

### Nested Objects and Arrays

```rust
use nail::parse_json;

let complex_json = r#"
{
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ],
    "metadata": {
        "total": 2,
        "active": true
    }
}
"#;

match parse_json(complex_json) {
    Ok(document) => {
        // Navigate the nested structure
        if let Some(object) = document.as_object() {
            println!("Successfully parsed complex JSON with {} top-level keys", object.len());
        }
    }
    Err(error) => eprintln!("Error: {}", error),
}
```

## API Reference

### Functions

#### `parse_json(json_string: &str) -> Result<JsonDocument, String>`

Parses a JSON string and returns a `JsonDocument` or an error message.

### Types

#### `JsonDocument`

Represents the root of a JSON document:
- `JsonDocument::Object(HashMap<String, JsonValue>)` - JSON object
- `JsonDocument::Array(Vec<JsonValue>)` - JSON array

Methods:
- `as_object(&self) -> Option<&HashMap<String, JsonValue>>` - Get as object if it's an object
- `as_array(&self) -> Option<&Vec<JsonValue>>` - Get as array if it's an array

#### `JsonValue`

Represents any JSON value:
- `JsonValue::Null` - JSON null
- `JsonValue::Boolean(bool)` - JSON boolean
- `JsonValue::Number(Number)` - JSON number
- `JsonValue::String(String)` - JSON string
- `JsonValue::Document(Box<JsonDocument>)` - Nested object or array

#### `Number`

Represents JSON numbers:
- `Number::Int(i32)` - Integer values
- `Number::Float(f64)` - Floating-point values

## Supported Escape Sequences

The parser supports all standard JSON escape sequences:

- `\"` - Double quote
- `\\` - Backslash
- `\/` - Forward slash
- `\b` - Backspace
- `\f` - Form feed
- `\n` - Newline
- `\r` - Carriage return
- `\t` - Tab
- `\uXXXX` - Unicode character (where XXXX is a 4-digit hexadecimal number)

## Error Handling

The parser provides descriptive error messages for common issues:

- Invalid JSON structure
- Malformed strings (unclosed quotes)
- Invalid escape sequences
- Invalid numbers
- Unexpected tokens

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```