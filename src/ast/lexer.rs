use std::fmt::{self};

use crate::KEYWORDS;

#[derive(Clone)]
pub enum LexerTokenType {
    FunctionCall,
    LetKeyword,
    IfKeyword,
    TrueKeyword,
    FalseKeyword,
    Identifier,
    AssignmentOperator,
    AddOperator,
    StringLiteral,
    Number,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    EndOfStatement,
    Any, // possible tokens or no tokens
    Unknown,
}

impl fmt::Display for LexerTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerTokenType::FunctionCall => write!(f, "FunctionCall"),
            LexerTokenType::LetKeyword => write!(f, "LetKeyword"),
            LexerTokenType::IfKeyword => write!(f, "IfKeyword"),
            LexerTokenType::TrueKeyword => write!(f, "TrueKeyword"),
            LexerTokenType::FalseKeyword => write!(f, "FalseKeyword"),
            LexerTokenType::Identifier => write!(f, "Identifier"),
            LexerTokenType::AssignmentOperator => write!(f, "AssignmentOperator"),
            LexerTokenType::AddOperator => write!(f, "AddOperator"),
            LexerTokenType::StringLiteral => write!(f, "StringLiteral"),
            LexerTokenType::Number => write!(f, "Number"),
            LexerTokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            LexerTokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            LexerTokenType::OpenCurlyBrace => write!(f, "OpenCurlyBrace"),
            LexerTokenType::CloseCurlyBrace => write!(f, "CloseCurlyBrace"),
            LexerTokenType::Comma => write!(f, "Comma"),
            LexerTokenType::EndOfStatement => write!(f, "EndOfStatement"),
            LexerTokenType::Any => write!(f, "Any"),
            LexerTokenType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl PartialEq for LexerTokenType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

#[derive(Clone)]
pub struct LexerToken {
    pub token_type: LexerTokenType,
    pub value: String,
}

impl LexerToken {
    fn new(token_type: LexerTokenType, value: String) -> LexerToken {
        LexerToken { token_type, value }
    }
}

impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.token_type, self.value)
    }
}

pub fn lex(source: String) -> Vec<LexerToken> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut is_comment = 0; // inside a comment

    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces
    while let Some(c) = chars.next() {
        // inside comment
        if is_comment > 1 {
            if c == '\n' {
                is_comment = 0;
            }
        // normal mode
        } else {
            match c {
                // a quote
                '"' => {
                    current_token.push(c);

                    if is_string {
                        tokens.push(token_with_type(current_token));
                        current_token = String::new();
                    }

                    is_string = !is_string;
                }
                // comments
                '/' => {
                    is_comment += 1;
                }
                // special characters
                '(' | ')' | '{' | '}' | ',' | ';' => {
                    if is_string {
                        current_token.push(c);
                    } else {
                        if current_token.len() > 0 {
                            tokens.push(token_with_type(current_token)); // push previous token
                            current_token = String::new();
                        }
                        tokens.push(token_with_type(c.to_string())); // push current
                    }
                }
                '=' | '+' => {
                    if is_string {
                        current_token.push(c);
                    } else {
                        tokens.push(token_with_type(c.to_string()));
                        current_token = String::new();
                    }
                }
                // whitespace types
                ' ' | '\n' | '\t' => {
                    if is_string {
                        current_token.push(c);
                    } else if keywords.contains(&current_token.as_str()) {
                        tokens.push(token_with_type(current_token));
                        current_token = String::new();
                    } else if current_token.len() > 0 {
                        // if not empty
                        tokens.push(token_with_type(current_token));
                        current_token = String::new();
                    }
                }
                // accumulating numbers
                _ if c.is_numeric() && current_token.chars().all(|char| char.is_numeric()) => {
                    if chars.peek().is_some_and(|char| char.is_numeric()) {
                        current_token.push(c);
                    } else {
                        current_token.push(c);
                        tokens.push(token_with_type(current_token));
                        current_token = String::new();
                    }
                }
                // characters that are not whitespace
                _ if is_string || !c.is_whitespace() => {
                    // check for numeric strings
                    current_token.push(c);
                }
                _ => {}
            }
        }
    }

    return tokens;
}

// Also, it doesn't handle the last token if it's not followed by whitespace.

fn token_with_type(token: String) -> LexerToken {
    match token.as_str() {
        "print" => LexerToken::new(LexerTokenType::FunctionCall, token),
        "let" => LexerToken::new(LexerTokenType::LetKeyword, token),
        "if" => LexerToken::new(LexerTokenType::IfKeyword, token),
        "true" => LexerToken::new(LexerTokenType::TrueKeyword, token),
        "false" => LexerToken::new(LexerTokenType::FalseKeyword, token),
        "(" => LexerToken::new(LexerTokenType::OpenParenthesis, token),
        ")" => LexerToken::new(LexerTokenType::CloseParenthesis, token),
        "{" => LexerToken::new(LexerTokenType::OpenCurlyBrace, token),
        "}" => LexerToken::new(LexerTokenType::CloseCurlyBrace, token),
        "," => LexerToken::new(LexerTokenType::Comma, token),
        ";" => LexerToken::new(LexerTokenType::EndOfStatement, token),
        "=" => LexerToken::new(LexerTokenType::AssignmentOperator, token),
        "+" => LexerToken::new(LexerTokenType::AddOperator, token),
        _ if token.chars().next() == Some('"') && token.chars().last() == Some('"') => {
            LexerToken::new(LexerTokenType::StringLiteral, token)
        }
        _ if token.chars().all(|c| c.is_numeric()) => {
            LexerToken::new(LexerTokenType::Number, token)
        }
        _ if is_identifier(token.clone()) => LexerToken::new(LexerTokenType::Identifier, token),
        _ => LexerToken::new(LexerTokenType::Unknown, token),
    }
}

fn is_identifier(token: String) -> bool {
    let mut chars = token.chars();
    chars
        .next()
        .is_some_and(|first_char| first_char.is_alphabetic() || first_char == '_')
        && chars.all(|char| char.is_alphanumeric() || char == '_')
}
