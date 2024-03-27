use std::fmt;

use crate::KEYWORDS;

#[derive(Clone, Debug)]
pub enum LexerTokenType {
    FunctionCall,
    LetKeyword,
    FnKeyword,
    IfKeyword,
    TrueKeyword,
    FalseKeyword,
    Identifier,
    AssignmentOperator,
    AddOperator,
    SubtractOperator,
    MultiplyOperator,
    DivideOperator,
    StringLiteral,
    Number,
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    EndOfStatement,
    Unknown,
}

impl fmt::Display for LexerTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerTokenType::FunctionCall => write!(f, "FunctionCall"),
            LexerTokenType::LetKeyword => write!(f, "LetKeyword"),
            LexerTokenType::FnKeyword => write!(f, "FnKeyword"),
            LexerTokenType::IfKeyword => write!(f, "IfKeyword"),
            LexerTokenType::TrueKeyword => write!(f, "TrueKeyword"),
            LexerTokenType::FalseKeyword => write!(f, "FalseKeyword"),
            LexerTokenType::Identifier => write!(f, "Identifier"),
            LexerTokenType::AssignmentOperator => write!(f, "AssignmentOperator"),
            LexerTokenType::AddOperator => write!(f, "AddOperator"),
            LexerTokenType::SubtractOperator => write!(f, "SubtractOperator"),
            LexerTokenType::MultiplyOperator => write!(f, "MultiplyOperator"),
            LexerTokenType::DivideOperator => write!(f, "DivideOperator"),
            LexerTokenType::StringLiteral => write!(f, "StringLiteral"),
            LexerTokenType::Number => write!(f, "Number"),
            LexerTokenType::OpenParenthesis => write!(f, "OpenParenthesis"),
            LexerTokenType::CloseParenthesis => write!(f, "CloseParenthesis"),
            LexerTokenType::OpenCurlyBrace => write!(f, "OpenCurlyBrace"),
            LexerTokenType::CloseCurlyBrace => write!(f, "CloseCurlyBrace"),
            LexerTokenType::Comma => write!(f, "Comma"),
            LexerTokenType::EndOfStatement => write!(f, "EndOfStatement"),
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
    pub line: usize,
    pub at: usize,
}

impl LexerToken {
    fn new(token_type: LexerTokenType, value: String, line: usize, at: usize) -> LexerToken {
        LexerToken {
            token_type,
            value,
            line,
            at,
        }
    }
}

impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} (line: {}, char: {})",
            self.token_type, self.value, self.line, self.at
        )
    }
}

pub fn lex(source: String) -> Vec<LexerToken> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut is_comment = false; // inside a comment

    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces
    let mut char_counter = 1; // all module chars number
    let mut line_char_counter = 1; // current line chars number
    let mut line_counter = 1; // track current line

    while let Some(c) = chars.next() {
        // inside comment
        if is_comment {
            if c == '\n' {
                is_comment = false;
                line_counter += 1;
                line_char_counter = 0;
            }
        // normal mode
        } else {
            match c {
                // a quote
                '"' => {
                    current_token.push(c);

                    if is_string {
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                    }

                    is_string = !is_string;
                }
                // comments & divide operator
                '/' => {
                    if let Some(next) = chars.peek() {
                        if c == '/' && next == &'/' {
                            is_comment = true;
                        } else {
                            if is_string {
                                current_token.push(c);
                            } else {
                                tokens.push(token_with_type(
                                    c.to_string(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token = String::new();
                            }
                        }
                    }
                }
                '=' | '+' | '-' | '*' => {
                    if is_string {
                        current_token.push(c);
                    } else {
                        tokens.push(token_with_type(
                            c.to_string(),
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                    }
                }
                // special characters
                '(' | ')' | '{' | '}' | ',' | ';' => {
                    if is_string {
                        current_token.push(c);
                    } else {
                        if current_token.len() > 0 {
                            tokens.push(token_with_type(
                                current_token,
                                line_counter,
                                line_char_counter - 1,
                            )); // push previous token, - 1 since is the previous
                            current_token = String::new();
                        }
                        tokens.push(token_with_type(
                            c.to_string(),
                            line_counter,
                            line_char_counter,
                        ));
                        // push current
                    }
                }
                // whitespace types
                ' ' | '\n' | '\t' => {
                    if is_string {
                        current_token.push(c);
                    } else if keywords.contains(&current_token.as_str()) {
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                    } else if current_token.len() > 0 {
                        // if not empty
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                    }

                    if c == '\n' {
                        line_counter += 1;
                        line_char_counter = 0;
                    }
                }
                // accumulating numbers
                _ if c.is_numeric() && current_token.chars().all(|char| char.is_numeric()) => {
                    if chars.peek().is_some_and(|char| char.is_numeric()) {
                        current_token.push(c);
                    } else {
                        current_token.push(c);
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
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

        // last character in the source code
        if char_counter == source.len() && current_token.len() > 0 {
            print!("hola");
            tokens.push(token_with_type(
                current_token,
                line_counter,
                line_char_counter,
            ));
            current_token = String::new()
        }

        // keep track of current char index
        line_char_counter += 1;
        char_counter += 1;
    }

    return tokens;
}

// Also, it doesn't handle the last token if it's not followed by whitespace.

fn token_with_type(token: String, line: usize, at: usize) -> LexerToken {
    match token.as_str() {
        "print" => LexerToken::new(LexerTokenType::FunctionCall, token, line, at),
        "fn" => LexerToken::new(LexerTokenType::FnKeyword, token, line, at),
        "let" => LexerToken::new(LexerTokenType::LetKeyword, token, line, at),
        "if" => LexerToken::new(LexerTokenType::IfKeyword, token, line, at),
        "true" => LexerToken::new(LexerTokenType::TrueKeyword, token, line, at),
        "false" => LexerToken::new(LexerTokenType::FalseKeyword, token, line, at),
        "(" => LexerToken::new(LexerTokenType::OpenParenthesis, token, line, at),
        ")" => LexerToken::new(LexerTokenType::CloseParenthesis, token, line, at),
        "{" => LexerToken::new(LexerTokenType::OpenCurlyBrace, token, line, at),
        "}" => LexerToken::new(LexerTokenType::CloseCurlyBrace, token, line, at),
        "," => LexerToken::new(LexerTokenType::Comma, token, line, at),
        ";" => LexerToken::new(LexerTokenType::EndOfStatement, token, line, at),
        "=" => LexerToken::new(LexerTokenType::AssignmentOperator, token, line, at),
        "+" => LexerToken::new(LexerTokenType::AddOperator, token, line, at),
        "-" => LexerToken::new(LexerTokenType::SubtractOperator, token, line, at),
        "*" => LexerToken::new(LexerTokenType::MultiplyOperator, token, line, at),
        "/" => LexerToken::new(LexerTokenType::DivideOperator, token, line, at),
        _ if token.chars().next() == Some('"') && token.chars().last() == Some('"') => {
            LexerToken::new(LexerTokenType::StringLiteral, token, line, at)
        }
        _ if token.chars().all(|c| c.is_numeric()) => {
            LexerToken::new(LexerTokenType::Number, token, line, at)
        }
        _ if is_identifier(token.clone()) => {
            LexerToken::new(LexerTokenType::Identifier, token, line, at)
        }
        _ => LexerToken::new(LexerTokenType::Unknown, token, line, at),
    }
}

fn is_identifier(token: String) -> bool {
    let mut chars = token.chars();
    chars
        .next()
        .is_some_and(|first_char| first_char.is_alphabetic() || first_char == '_')
        && chars.all(|char| char.is_alphanumeric() || char == '_')
}
