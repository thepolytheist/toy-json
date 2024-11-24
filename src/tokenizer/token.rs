/// Lexical tokens for parsing JSON.
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Identifier(String)
}

impl Token {
    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Token::Identifier(ident) => Some(ident),
            _ => None
        }
    }
}