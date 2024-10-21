use std::{collections::HashMap, iter::Peekable, slice::Iter};

use crate::{
    tokenizer::Token,
    types::{JsonDocument, JsonValue},
};

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn parse_tokens(&self) -> Result<JsonDocument, String> {
        let mut token_iter = self.tokens.iter().peekable();

        match token_iter.next() {
            Some(Token::LeftBrace) => self.parse_object(&mut token_iter),
            Some(Token::LeftBracket) => self.parse_array(&mut token_iter),
            _ => Err("Invalid JSON".to_string()),
        }
    }

    fn parse_value(&self, token_iter: &mut Peekable<Iter<Token>>) -> Result<JsonValue, String> {
        match token_iter.next() {
            Some(Token::LeftBrace) => {
                let obj = self.parse_object(token_iter)?;
                return Ok(JsonValue::Document(Box::new(obj)));
            }
            Some(Token::LeftBracket) => {
                let arr = self.parse_array(token_iter)?;
                return Ok(JsonValue::Document(Box::new(arr)));
            }
            Some(Token::Null) => {
                return Ok(JsonValue::Null);
            }
            Some(Token::Number(n)) => {
                return Ok(JsonValue::Number(n.clone()));
            }
            Some(Token::String(s)) => {
                return Ok(JsonValue::String(s.clone()));
            }
            Some(Token::Boolean(b)) => {
                return Ok(JsonValue::Boolean(b.clone()));
            }
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn parse_object(&self, token_iter: &mut Peekable<Iter<Token>>) -> Result<JsonDocument, String> {
        let mut object: HashMap<String, crate::types::JsonValue> = std::collections::HashMap::new();

        loop {
            match token_iter.next() {
                Some(Token::RightBrace) => return Ok(JsonDocument::Object(object)),
                Some(Token::String(key)) => {
                    if let Some(Token::Colon) = token_iter.next() {
                        let value = self.parse_value(token_iter)?;
                        object.insert(key.clone(), value);

                        match token_iter.next() {
                            Some(Token::Comma) => continue,
                            Some(Token::RightBrace) => return Ok(JsonDocument::Object(object)),
                            _ => return Err("Unexpected token".to_string()),
                        }
                    } else {
                        return Err("Unexpected token".to_string());
                    }
                }
                _ => return Err("Unexpected token".to_string()),
            }
        }
    }

    fn parse_array(&self, token_iter: &mut Peekable<Iter<Token>>) -> Result<JsonDocument, String> {
        let mut arr: Vec<JsonValue> = Vec::new();

        loop {
            match token_iter.peek() {
                Some(Token::RightBracket) => {
                    token_iter.next();
                    return Ok(JsonDocument::Array(arr));
                }
                Some(_) => {
                    let value = self.parse_value(token_iter)?;
                    arr.push(value);

                    match token_iter.next() {
                        Some(Token::Comma) => continue,
                        Some(Token::RightBracket) => return Ok(JsonDocument::Array(arr)),
                        _ => return Err("Unexpected token".to_string()),
                    }
                }
                None => {
                    return Err("Unexpected end of array".to_string());
                }
            }
        }
    }
}
