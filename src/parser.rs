use std::{collections::HashMap, error::Error, fmt::Display};

use json_value::JsonValue;

use crate::tokenizer::{token::Token, tokenize};

pub mod json_value;

#[derive(Debug)]
pub struct ParseError(String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl Error for ParseError {}

/// Converts a stream of [Token]s into a [JsonValue::Object].
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    /// Main entry point for [Parser]. Expects a single root [JsonValue::Object].
    pub fn parse_string(json_string: String) -> Result<JsonValue, ParseError> {
        let mut parser = Parser::new(tokenize(&json_string).unwrap());
        let json_value = parser.parse_object();
        if parser.pos != parser.tokens.len() {
            return Err(ParseError("Unexpected tokens after root object.".into()))
        }
        json_value
    }

    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn next_token_is(&self, expected: Token) -> bool {
        self.tokens.get(self.pos) == Some(&expected)
    }

    fn next_token_is_not(&self, undesired: Token) -> bool {
        self.tokens.get(self.pos) != Some(&undesired)
    }

    fn expect_identifier(&mut self) -> Result<&str, ParseError> {
        self.expect_matching(|t| matches!(t, Token::Identifier(_)))
            .map(|t| t.as_identifier().unwrap())
    }  

    fn expect_eq(&mut self, token: Token) -> Result<&Token, ParseError> {
        self.expect_matching(|t| *t == token)
    }

    fn expect_matching(&mut self, f: impl Fn(&Token) -> bool) -> Result<&Token, ParseError> {
        match self.next_token() {
            Some(token) if f(token) => Ok(token),
            Some(token) => Err(ParseError(format!("unexpected token: {:?}", token))),
            None => Err(ParseError("Unexpected end of input".into()))
        }
    }

    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            // FIXME: Would be nice to use advance() here.
            self.pos += 1;
        }
        token
    }

    fn peek_next_token(&self) -> Result<&Token, ParseError> {
        match self.tokens.get(self.pos) {
            Some(token) => Ok(token),
            _ => Err(ParseError("Unexpected end of input".into()))
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        self.expect_eq(Token::LeftBrace)?;
        let members = self.parse_members()?;
        self.expect_eq(Token::RightBrace)?;

        Ok(JsonValue::Object { members })
    }

    fn parse_members(&mut self) -> Result<HashMap<String, JsonValue>, ParseError> {
        let mut members = HashMap::new();

        while self.next_token_is_not(Token::RightBrace) {
            let (key, value) = self.parse_member()?;
            members.insert(key, value);
            if self.next_token_is(Token::Comma) {
                self.advance();
            }
        }

        Ok(members)
    }

    fn parse_member(&mut self) -> Result<(String, JsonValue), ParseError> {
        let key = self.parse_key()?;
        self.expect_eq(Token::Colon)?;
        let value = self.parse_value()?;
        
        Ok((key, value))
    }

    /// Consumes the next token, expecting a [Token::Identifier], and returns as a [String]. Otherwise, returns [ParseError].
    fn parse_key(&mut self) -> Result<String, ParseError> {
        // TODO: This method really just exists to provide a lexical scope for the return value of the identifier retrieval.
        // Otherwise, the calling site was trying to consume the key and value in the same loop, and this caused a double mutable borrow.
        // Could potentially make this cleaner with a refactor.
        Ok(self.expect_identifier()?.into())
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        // FIXME: This is only returning JsonValue::Object, Array, or String, but it needs to check all the types.
        match self.peek_next_token()? {
            &Token::LeftBrace => Ok(self.parse_object()?),
            &Token::LeftBracket => Ok(self.parse_array()?),
            _ => Ok(JsonValue::String(self.expect_identifier()?.into()))
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        self.expect_eq(Token::LeftBracket)?;
        let values = self.parse_array_values()?;
        self.expect_eq(Token::RightBracket)?;

        Ok(JsonValue::Array(values))
    }

    fn parse_array_values(&mut self) -> Result<Vec<JsonValue>, ParseError> {
        let mut values = Vec::new();
        while self.next_token_is_not(Token::RightBracket) {
            values.push(self.parse_value()?);
            if self.next_token_is(Token::Comma) {
                self.advance();
            }
        }

        Ok(values)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}
