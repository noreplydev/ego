use std::collections::HashSet;

pub fn lex(source: String) -> Vec<String> {
    let keywords: [&str; 6] = ["print", "if", "else", "while", "for", "fn"];
    let keywords: HashSet<String> = keywords.into_iter().map(|k| k.to_string()).collect();

    let mut tokens: Vec<String> = Vec::new();

    let source = source.trim(); // remove leading and trailing whitespaces
    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag

    for i in 0..source.len() {
        if i < source.len() {
            let c = source
                .chars()
                .nth(i)
                .expect("[goru] Error string index out of bounds.");

            match c {
                // a quote
                '"' => {
                    current_token.push(c);

                    if is_string {
                        tokens.push(current_token);
                        current_token = String::new();
                    }

                    is_string = !is_string;
                }
                // comments
                '/' => {}
                // whitespace types
                ' ' | '\n' | '\t' => {
                    if keywords.contains(&current_token) {
                        tokens.push(current_token);

                        current_token = String::new();
                    }
                }
                // characters
                _ if is_string || !c.is_whitespace() => {
                    current_token.push(c);
                }
                _ => {}
            }
        } else {
            return tokens;
        }
    }

    return tokens;
}
