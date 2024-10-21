#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    pub input_string: &'a str,
}

use crate::types::Number;
use std::{iter::Peekable, str::Chars};

impl<'a> Tokenizer<'a> {
    pub fn tokenize_json(&self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = self.input_string.chars().peekable();

        while let Some(c) = chars.peek() {
            match c {
                '{' => {
                    tokens.push(Token::LeftBrace);
                    chars.next();
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    chars.next();
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    chars.next();
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    chars.next();
                }
                ':' => {
                    tokens.push(Token::Colon);
                    chars.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    chars.next();
                }
                'n' => {
                    if self.try_tokenize_null(&mut chars) {
                        tokens.push(Token::Null);
                    } else {
                        return Err("Invalid JSON".into());
                    }
                }
                't' => {
                    if self.try_tokenize_true(&mut chars) {
                        tokens.push(Token::Boolean(true));
                    } else {
                        return Err("Invalid JSON".into());
                    }
                }
                'f' => {
                    if self.try_tokenize_false(&mut chars) {
                        tokens.push(Token::Boolean(false));
                    } else {
                        return Err("Invalid JSON".into());
                    }
                }
                '"' => {
                    match self.try_tokenize_string(&mut chars) {
                        Ok(result) => {
                            tokens.push(Token::String(result));
                        }
                        Err(err) => return Err(err),
                    };
                }
                '0'..='9' | '-' => {
                    match self.try_tokenize_number(&mut chars) {
                        Ok(result) => {
                            tokens.push(Token::Number(result));
                        }
                        Err(err) => return Err(err),
                    };
                }
                ' ' | '\n' | '\r' => {
                    chars.next();
                    continue;
                }
                _ => return Err("Invalid JSON".into()),
            };
        }

        Ok(tokens)
    }

    fn match_exact_word(&self, chars: &mut Peekable<Chars>, word: &str) -> bool {
        let length = word.len();

        chars.take(length).eq(word.chars())
    }

    fn try_tokenize_null(&self, chars: &mut Peekable<Chars>) -> bool {
        self.match_exact_word(chars, "null")
    }

    fn try_tokenize_true(&self, chars: &mut Peekable<Chars>) -> bool {
        self.match_exact_word(chars, "true")
    }

    fn try_tokenize_false(&self, chars: &mut Peekable<Chars>) -> bool {
        self.match_exact_word(chars, "false")
    }

    fn try_tokenize_string(&self, chars: &mut Peekable<Chars>) -> Result<String, String> {
        // skip the opening double quote
        chars.next();

        let mut extracted_string = String::new();

        for next_char in chars.by_ref() {
            match next_char {
                '"' => return Ok(extracted_string),
                _ => extracted_string.push(next_char),
            }
        }

        Err("EOF reached when parsing string".into())
    }

    fn try_tokenize_number(&self, chars: &mut Peekable<Chars>) -> Result<Number, String> {
        const ERROR_MSG: &str = "Invalid number";
        let mut extracted_string = String::new();
        let mut has_dot = false;
        let mut has_number = false;

        if chars.peek() == Some(&'-') {
            extracted_string.push('-');
            chars.next();
        }

        for next_char in chars.by_ref() {
            match next_char {
                '.' => {
                    // number cannot have more than 1 .
                    if has_dot {
                        return Err(ERROR_MSG.into());
                    }
                    // must have number before .
                    if !has_number {
                        return Err(ERROR_MSG.into());
                    }
                    extracted_string.push(next_char);
                    has_dot = true;
                }
                '0'..='9' => {
                    extracted_string.push(next_char);
                    has_number = true;
                }
                ',' | '\n' | '\r' | ' ' => {
                    return self.parse_number(&extracted_string, has_dot);
                }
                '-' => return Err(ERROR_MSG.into()),
                _ => return Err(ERROR_MSG.into()),
            }
        }

        if has_number {
            self.parse_number(&extracted_string, has_dot)
        } else {
            Err(ERROR_MSG.into())
        }
    }

    fn parse_number(&self, s: &str, is_float: bool) -> Result<Number, String> {
        if is_float {
            s.parse::<f64>()
                .map(Number::Float)
                .map_err(|_| "Invalid float".into())
        } else {
            s.parse::<i32>()
                .map(Number::Int)
                .map_err(|_| "Invalid integer".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn test_tokenize_json_simplest() {
        let input = r#"{"foo": "bar"}"#;
        let tokenizer = Tokenizer {
            input_string: input,
        };

        match tokenizer.tokenize_json() {
            Ok(result) => {
                let expected = vec![
                    Token::LeftBrace,
                    Token::String(String::from("foo")),
                    Token::Colon,
                    Token::String(String::from("bar")),
                    Token::RightBrace,
                ];

                assert_eq!(result, expected);
            }
            Err(e) => panic!("should not throw this error: {:?}", e),
        }
    }

    #[test]
    fn test_try_tokenize_null() {
        let input = "null";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_null(&mut chars), true);
    }

    #[test]
    fn test_try_tokenize_null_return_false() {
        let input = "none";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_null(&mut chars), false);
    }

    #[test]
    fn test_try_tokenize_true() {
        let input = "true";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_true(&mut chars), true);
    }

    #[test]
    fn test_try_tokenize_true_return_false() {
        let input = "turtle";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_true(&mut chars), false);
    }

    #[test]
    fn test_try_tokenize_false_return() {
        let input = "false";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_false(&mut chars), true);
    }

    #[test]
    fn test_try_tokenize_false_with_suffix() {
        let input = "false, ";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_false(&mut chars), true);
    }

    #[test]
    fn test_try_tokenize_false_return_false() {
        let input = "f";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        assert_eq!(tokenizer.try_tokenize_false(&mut chars), false);
    }

    #[test]
    fn test_try_tokenize_string() {
        let input = r#""Hello World!""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(result) => {
                assert_eq!(result, "Hello World!");
            }
            Err(_) => panic!("Expect success tokenize string"),
        }
    }

    #[test]
    fn test_try_tokenize_string_unclosed() {
        let input = r#""Hello World!"#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(_) => {
                panic!("Expect error returned for unclosed string");
            }
            Err(e) => assert_eq!(e, "EOF reached when parsing string"),
        }
    }

    #[test]
    fn test_try_tokenize_number_with_integer() {
        let input = "23";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(result) => assert_eq!(result, Number::Int(23)),
            Err(_) => panic!("Expect not to throw error"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_float() {
        let input = "52.1985";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(result) => assert_eq!(result, Number::Float(52.1985)),
            Err(_) => panic!("Expect not to throw error"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_negative_integer() {
        let input = "-11";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(result) => assert_eq!(result, Number::Int(-11)),
            Err(_) => panic!("Expect not to throw error"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_negative_float() {
        let input = "-47.9999999";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(result) => assert_eq!(result, Number::Float(-47.9999999)),
            Err(_) => panic!("Expect not to throw error"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_negative_float2() {
        let input = "-0.33";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(result) => assert_eq!(result, Number::Float(-0.33)),
            Err(_) => panic!("Expect not to throw error"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_2_dots() {
        let input = "-52.33.3";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(_) => panic!("Expect to throw error"),
            Err(err) => assert_eq!(err, "Invalid number"),
        };
    }

    #[test]
    fn test_try_tokenize_number_with_2_minus() {
        let input = "-52-11";
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_number(&mut chars) {
            Ok(_) => panic!("Expect to throw error"),
            Err(err) => assert_eq!(err, "Invalid number"),
        };
    }
}
