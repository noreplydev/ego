use super::lexer_types::{LexerTokenType,LexerToken};
use regex::Regex;



const KEYWORDS: [&str; 14] = [
    "fn", "let", "if", "else", "while", "true", "false", "import", "return", "break", "nothing",
    "string", "number", "bool",
];

pub fn lex(source: String) -> Vec<LexerToken> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut is_comment = false; // inside a comment
    let mut is_float = false; // inside a float value

    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces
    let mut char_counter = 1; // all module chars number
    let mut line_char_counter = 1; // current line chars number
    let mut line_counter = 1; // track current line

    while let Some(c) = chars.next() {
        if is_comment {
            if c == '\n' {
                is_comment = false;
                line_counter += 1;
                line_char_counter = 0;
            }
        } else if is_string {
            if c == '"' {
                current_token.push(c);
                is_string = false;
            } else {
                current_token.push(c);
            }
        } else if is_float {
            if c.is_numeric() && chars.peek().is_some_and(|char| char.is_numeric()) {
                current_token.push(c);
            } else {
                current_token.push(c);
                is_float = false;
            }
        } else {
            // normal mode
            match c {
                // a quote
                '"' => {
                    if current_token.len() > 0 {
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                    }

                    current_token.push(c);
                    is_string = !is_string;
                }
                // comments & divide operator
                '/' => {
                    if let Some(next) = chars.peek() {
                        if c == '/' && next == &'/' {
                            is_comment = true;
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
                            current_token = String::new();
                        }
                    }
                }
                // expressions characters
                '=' => {
                    if let Some(next) = chars.peek() {
                        match next {
                            '=' => {
                                // for '=='
                                chars.next(); // Consume the '='
                                current_token.push(c);
                                current_token.push('=');
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                            _ => {
                                // for '='
                                current_token.push(c);
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                        }
                    }
                }
                '>' => {
                    if let Some(next) = chars.peek() {
                        match next {
                            '=' => {
                                // for '>='
                                chars.next(); // Consume the '='
                                current_token.push(c);
                                current_token.push('=');
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                            _ => {
                                // for '>'
                                current_token.push(c);
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                        }
                    }
                }
                '<' => {
                    if let Some(next) = chars.peek() {
                        match next {
                            '=' => {
                                // for '>='
                                chars.next(); // Consume the '='
                                current_token.push(c);
                                current_token.push('=');
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                            _ => {
                                // for '>'
                                current_token.push(c);
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                        }
                    }
                }
                '!' => {
                    if let Some(next) = chars.peek() {
                        match next {
                            '=' => {
                                // for '!='
                                chars.next(); // Consume the '='
                                current_token.push(c);
                                current_token.push('=');
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                            _ => {
                                // for '!'
                                current_token.push(c);
                                tokens.push(token_with_type(
                                    current_token.clone(),
                                    line_counter,
                                    line_char_counter,
                                ));
                                current_token.clear();
                            }
                        }
                    }
                }
                '+' | '-' | '*' | '|' | '&' => {
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
                    current_token = String::new();
                }
                // special characters
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | ';' | ':' => {
                    if current_token.len() > 0 {
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter - 1,
                        )); // push previous token, - 1 since is the previous
                        current_token = String::new();
                    }
                    // push current
                    tokens.push(token_with_type(
                        c.to_string(),
                        line_counter,
                        line_char_counter,
                    ));
                }
                // dot and float
                '.' => {
                    if current_token.chars().all(|char| char.is_numeric()) {
                        current_token.push(c);
                        is_float = !is_float;
                    } else {
                        tokens.push(token_with_type(
                            current_token,
                            line_counter,
                            line_char_counter,
                        ));
                        current_token = String::new();
                        current_token.push(c);
                    }
                }
                // whitespace types
                ' ' | '\n' | '\t' => {
                    if keywords.contains(&current_token.as_str()) {
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
                    if chars
                        .peek()
                        .is_some_and(|char| char.is_numeric() || char == &'.')
                    {
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
                _ if !c.is_whitespace() => {
                    // check for numeric strings
                    current_token.push(c);
                }
                _ => {}
            }
        }

        // last character in the source code
        if char_counter == source.len() && current_token.len() > 0 {
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

fn token_with_type(token: String, line: usize, at: usize) -> LexerToken {
    match token.as_str() {
        "import" => LexerToken::new(LexerTokenType::ImportKeyword, token, line, at),
        "fn" => LexerToken::new(LexerTokenType::FnKeyword, token, line, at),
        "while" => LexerToken::new(LexerTokenType::WhileKeyword, token, line, at),
        "let" => LexerToken::new(LexerTokenType::LetKeyword, token, line, at),
        "if" => LexerToken::new(LexerTokenType::IfKeyword, token, line, at),
        "else" => LexerToken::new(LexerTokenType::ElseKeyword, token, line, at),
        "true" => LexerToken::new(LexerTokenType::TrueKeyword, token, line, at),
        "false" => LexerToken::new(LexerTokenType::FalseKeyword, token, line, at),
        "return" => LexerToken::new(LexerTokenType::ReturnKeyword, token, line, at),
        "break" => LexerToken::new(LexerTokenType::BreakKeyword, token, line, at),
        "nothing" => LexerToken::new(LexerTokenType::NothingKeyword, token, line, at),
        "string" => LexerToken::new(LexerTokenType::StringKeyword, token, line, at),
        "number" => LexerToken::new(LexerTokenType::NumberKeyword, token, line, at),
        "bool" => LexerToken::new(LexerTokenType::BoolKeyword, token, line, at),
        "(" => LexerToken::new(LexerTokenType::OpenParenthesis, token, line, at),
        ")" => LexerToken::new(LexerTokenType::CloseParenthesis, token, line, at),
        "{" => LexerToken::new(LexerTokenType::OpenCurlyBrace, token, line, at),
        "}" => LexerToken::new(LexerTokenType::CloseCurlyBrace, token, line, at),
        "[" => LexerToken::new(LexerTokenType::OpenSquareBracket, token, line, at),
        "]" => LexerToken::new(LexerTokenType::CloseSquareBracket, token, line, at),
        "." => LexerToken::new(LexerTokenType::Dot, token, line, at),
        ":" => LexerToken::new(LexerTokenType::Colon, token, line, at),
        "," => LexerToken::new(LexerTokenType::Comma, token, line, at),
        ";" => LexerToken::new(LexerTokenType::EndOfStatement, token, line, at),
        "!" => LexerToken::new(LexerTokenType::NotOperator, token, line, at),
        "!=" => LexerToken::new(LexerTokenType::NotEqualOperator, token, line, at),
        "=" => LexerToken::new(LexerTokenType::AssignmentOperator, token, line, at),
        "==" => LexerToken::new(LexerTokenType::EqualityOperator, token, line, at),
        "+" => LexerToken::new(LexerTokenType::AddOperator, token, line, at),
        "-" => LexerToken::new(LexerTokenType::SubtractOperator, token, line, at),
        "*" => LexerToken::new(LexerTokenType::MultiplyOperator, token, line, at),
        "|" => LexerToken::new(LexerTokenType::OrOperator, token, line, at),
        "&" => LexerToken::new(LexerTokenType::AmpersandOperator, token, line, at),
        "/" => LexerToken::new(LexerTokenType::DivideOperator, token, line, at),
        ">" => LexerToken::new(LexerTokenType::GreaterThanOperator, token, line, at),
        ">=" => LexerToken::new(LexerTokenType::GreaterThanOrEqualOperator, token, line, at),
        "<" => LexerToken::new(LexerTokenType::LessThanOperator, token, line, at),
        "<=" => LexerToken::new(LexerTokenType::LessThanOrEqualOperator, token, line, at),
        _ if token.chars().next() == Some('"') && token.chars().last() == Some('"') => {
            LexerToken::new(LexerTokenType::StringLiteral, token, line, at)
        }
        _ if is_number(&token) => LexerToken::new(LexerTokenType::Number, token, line, at),
        _ if is_identifier(&token) => LexerToken::new(LexerTokenType::Identifier, token, line, at),
        _ => LexerToken::new(LexerTokenType::Unknown, token, line, at),
    }
}

fn is_number(token: &String) -> bool {
    let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();
    if re.is_match(token.as_str()) {
        true
    } else {
        false
    }
}

fn is_identifier(token: &String) -> bool {
    let mut chars = token.chars();
    chars
        .next()
        .is_some_and(|first_char| first_char.is_alphabetic() || first_char == '_')
        && chars.all(|char| char.is_alphanumeric() || char == '_')
}
