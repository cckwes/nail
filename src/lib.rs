use parser::Parser;
use tokenizer::Tokenizer;
pub use types::JsonDocument;

mod parser;
mod tokenizer;
mod types;

pub fn parse_json(json_string: &str) -> Result<JsonDocument, String> {
    let tokenizer = Tokenizer {
        input_string: json_string,
    };
    let parser = Parser {
        tokens: tokenizer.tokenize_json()?,
    };

    return parser.parse_tokens();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use types::JsonValue;

    use super::*;

    #[test]
    fn test_parse_json_simplest() {
        let json_string = r#"{"foo": "bar"}"#;

        match parse_json(json_string) {
            Ok(result) => {
                println!("result is {:?}", result);

                let mut object_hash_map = HashMap::new();
                object_hash_map.insert("foo".to_string(), JsonValue::String("bar".to_string()));

                assert_eq!(result, JsonDocument::Object(object_hash_map))
            }
            Err(e) => panic!("Expect success json parsing, with error {:?}", e),
        }
    }
}
