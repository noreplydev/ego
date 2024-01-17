const KEYWORDS: [&str; 1] = ["print"]; // "if", "else", "while", "for", "fn"

pub fn lex(source: String) -> Vec<String> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<String> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces

    while let Some(&c) = chars.peek() {
        match c {
            // a quote
            '"' => {
                current_token.push(c);

                if is_string {
                    tokens.push(current_token);
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
                    tokens.push(current_token);

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
