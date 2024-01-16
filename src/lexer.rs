use std::collections::HashSet;
/* enum tokenTypes {
    Keyword, // print, if, else, while, for, ...
    Number,  // 123, 123.456
    String,  // "Hello world"
} */

pub fn lex(source: String) -> Vec<String> {
    let keywords: [&str; 6] = ["print", "if", "else", "while", "for", "fn"];
    let keywords: HashSet<String> = keywords.into_iter().map(|k| k.to_string()).collect();

    let mut tokens: Vec<String> = Vec::new();

    // lex here
    let source = source.trim(); // remove leading and trailing whitespaces

    let mut current_token = String::new();
    let mut is_string = false;

    for c in source.chars() {
        if c == '"' {
            current_token.push(c);

            if is_string {
                tokens.push(current_token);
                current_token = String::new();
            }

            is_string = !is_string;
        } else if is_string {
            current_token.push(c);
        } else if c == ' ' {
            if keywords.contains(&current_token) {
                tokens.push(current_token);
            }

            current_token = String::new();
        } else {
            current_token.push(c);
        }
    }

    return tokens;
}
