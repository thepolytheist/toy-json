use std::{error::Error, fmt::Display};

use token::Token;

pub mod token;

#[derive(Debug)]
pub struct TokenizeError(String);

impl Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tokenizing error: {}", self.0)
    }
}

impl Error for TokenizeError {}

/// Converts a &[str] into a vector of [Token]s. Otherwise, returns [TokenizeError].
pub fn tokenize(json_string: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut chars = json_string.chars().peekable();

    while let Some(c) = chars.next() {
        // FIXME: Does not handle number values, bools, or null.
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            c if c.is_whitespace() => continue,
            c if c == '"' => {
                let mut ident = String::new();
                while let Some(cc) = chars.next_if(|&cc| cc != '"') {
                    ident.push(cc);
                }

                if let None = chars.next_if(|&cc| cc == '"') {
                    return Err(TokenizeError("Expected \" to end string value.".into()))
                }

                tokens.push(Token::Identifier(ident));
            },
            _ => return Err(TokenizeError(format!("Unexpected character: {}", c)))
        }
    }

    Ok(tokens)
}