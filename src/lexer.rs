use std::fmt;

const KEYWORDS: [&str; 1] = ["print"]; // "if", "else", "while", "for", "fn"

enum TokenType {
    ExpressionStatement,
    StringLiteral,
    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ExpressionStatement => write!(f, "ExpressionStatement"),
            TokenType::StringLiteral => write!(f, "StringLiteral"),
            TokenType::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.token_type, self.value)
    }
}

pub fn lex(source: String) -> Vec<Token> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces

    while let Some(&c) = chars.peek() {
        match c {
            // a quote
            '"' => {
                current_token.push(c);

                if is_string {
                    tokens.push(token_with_type(current_token));
                    current_token = String::new();
                }

                is_string = !is_string;
                chars.next();
            }
            // comments
            '/' => {}
            // whitespace types
            ' ' | '\n' | '\t' => {
                if keywords.contains(&current_token.as_str()) {
                    tokens.push(token_with_type(current_token));

                    current_token = String::new();
                }
                chars.next();
            }
            // characters
            _ if is_string || !c.is_whitespace() => {
                current_token.push(c);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    return tokens;
}

// Also, it doesn't handle the last token if it's not followed by whitespace.

fn token_with_type(token: String) -> Token {
    match token.as_str() {
        "print" => Token::new(TokenType::ExpressionStatement, token),
        _ if token.chars().next() == Some('"') && token.chars().last() == Some('"') => {
            Token::new(TokenType::StringLiteral, token)
        }
        _ => Token::new(TokenType::Unknown, token),
    }
}
