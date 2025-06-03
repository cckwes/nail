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

        while let Some(next_char) = chars.next() {
            match next_char {
                '"' => return Ok(extracted_string),
                '\\' => {
                    // Handle escape sequences
                    match chars.next() {
                        Some('"') => extracted_string.push('"'),
                        Some('\\') => extracted_string.push('\\'),
                        Some('/') => extracted_string.push('/'),
                        Some('b') => extracted_string.push('\u{0008}'), // backspace
                        Some('f') => extracted_string.push('\u{000C}'), // form feed
                        Some('n') => extracted_string.push('\n'),
                        Some('r') => extracted_string.push('\r'),
                        Some('t') => extracted_string.push('\t'),
                        Some('u') => {
                            // Unicode escape sequence \uXXXX
                            let mut unicode_digits = String::new();
                            for _ in 0..4 {
                                match chars.next() {
                                    Some(c) if c.is_ascii_hexdigit() => unicode_digits.push(c),
                                    _ => return Err("Invalid unicode escape sequence".into()),
                                }
                            }
                            match u32::from_str_radix(&unicode_digits, 16) {
                                Ok(code_point) => {
                                    match char::from_u32(code_point) {
                                        Some(unicode_char) => extracted_string.push(unicode_char),
                                        None => return Err("Invalid unicode code point".into()),
                                    }
                                }
                                Err(_) => return Err("Invalid unicode escape sequence".into()),
                            }
                        }
                        Some(_) => return Err("Invalid escape sequence".into()),
                        None => return Err("EOF reached when parsing escape sequence".into()),
                    }
                }
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

        loop {
            match chars.peek() {
                Some('0'..='9') => {
                    extracted_string.push(chars.next().unwrap());
                    has_number = true;
                }
                Some('.') => {
                    // number cannot have more than 1 .
                    if has_dot {
                        return Err(ERROR_MSG.into());
                    }
                    // must have number before .
                    if !has_number {
                        return Err(ERROR_MSG.into());
                    }
                    extracted_string.push(chars.next().unwrap());
                    has_dot = true;
                }
                Some(',' | '\n' | '\r' | ' ' | '}' | ']') => {
                    return self.parse_number(&extracted_string, has_dot);
                }
                Some('-') => return Err(ERROR_MSG.into()),
                Some(_) => return Err(ERROR_MSG.into()),
                None => break,
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

    #[test]
    fn test_try_tokenize_string_with_escaped_quotes() {
        let input = r#""He said \"Hello World!\"""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(result) => {
                assert_eq!(result, r#"He said "Hello World!""#);
            }
            Err(_) => panic!("Expect success tokenize string with escaped quotes"),
        }
    }

    #[test]
    fn test_try_tokenize_string_with_escaped_backslash() {
        let input = r#""Path: C:\\Users\\test""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(result) => {
                assert_eq!(result, r"Path: C:\Users\test");
            }
            Err(_) => panic!("Expect success tokenize string with escaped backslash"),
        }
    }

    #[test]
    fn test_try_tokenize_string_with_newline_escape() {
        let input = r#""Line 1\nLine 2""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(result) => {
                assert_eq!(result, "Line 1\nLine 2");
            }
            Err(_) => panic!("Expect success tokenize string with newline escape"),
        }
    }

    #[test]
    fn test_try_tokenize_string_with_unicode_escape() {
        let input = r#""Unicode: \u0048\u0065\u006C\u006C\u006F""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(result) => {
                assert_eq!(result, "Unicode: Hello");
            }
            Err(_) => panic!("Expect success tokenize string with unicode escape"),
        }
    }

    #[test]
    fn test_try_tokenize_string_with_invalid_escape() {
        let input = r#""Invalid \x escape""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(_) => panic!("Expect error for invalid escape sequence"),
            Err(err) => assert_eq!(err, "Invalid escape sequence"),
        }
    }

    #[test]
    fn test_try_tokenize_string_with_incomplete_unicode() {
        let input = r#""Unicode: \u00""#;
        let tokenizer = Tokenizer {
            input_string: input,
        };
        let mut chars = input.chars().peekable();

        match tokenizer.try_tokenize_string(&mut chars) {
            Ok(_) => panic!("Expect error for incomplete unicode escape"),
            Err(err) => assert_eq!(err, "Invalid unicode escape sequence"),
        }
    }

    #[test]
    fn test_tokenize_json_with_number_at_end_of_object() {
        let input = r#"{"num":42}"#;
        let tokenizer = Tokenizer {
            input_string: input,
        };

        match tokenizer.tokenize_json() {
            Ok(result) => {
                let expected = vec![
                    Token::LeftBrace,
                    Token::String(String::from("num")),
                    Token::Colon,
                    Token::Number(Number::Int(42)),
                    Token::RightBrace,
                ];
                assert_eq!(result, expected);
            }
            Err(e) => panic!("should not throw this error: {:?}", e),
        }
    }

    #[test]
    fn test_tokenize_json_with_number_at_end_of_array() {
        let input = r#"[1,2,3]"#;
        let tokenizer = Tokenizer {
            input_string: input,
        };

        match tokenizer.tokenize_json() {
            Ok(result) => {
                let expected = vec![
                    Token::LeftBracket,
                    Token::Number(Number::Int(1)),
                    Token::Comma,
                    Token::Number(Number::Int(2)),
                    Token::Comma,
                    Token::Number(Number::Int(3)),
                    Token::RightBracket,
                ];
                assert_eq!(result, expected);
            }
            Err(e) => panic!("should not throw this error: {:?}", e),
        }
    }

    #[test]
    fn test_tokenize_json_with_float_at_end_of_object() {
        let input = r#"{"pi":3.14159}"#;
        let tokenizer = Tokenizer {
            input_string: input,
        };

        match tokenizer.tokenize_json() {
            Ok(result) => {
                let expected = vec![
                    Token::LeftBrace,
                    Token::String(String::from("pi")),
                    Token::Colon,
                    Token::Number(Number::Float(3.14159)),
                    Token::RightBrace,
                ];
                assert_eq!(result, expected);
            }
            Err(e) => panic!("should not throw this error: {:?}", e),
        }
    }
}
