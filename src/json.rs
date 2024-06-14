use std::str::Chars;

#[derive(Debug)]
pub enum JSONValue {
    Object(Vec<(String, JSONValue)>),
    Array(Vec<JSONValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum JSONToken {
    LBRACE,
    RBRACE,
    COMMA,
    QUOTE,
    LITERAL(String),
}

pub fn lex(input: String) -> Vec<JSONToken> {
    let mut tokens: Vec<JSONToken> = Vec::new();

    let mut characters = input.chars();
    let mut maybe_next_char = characters.next();

    while maybe_next_char.is_some() {
        let next_char = maybe_next_char.unwrap();

        if !next_char.is_whitespace() {
            match next_char {
                '{' => {
                    tokens.push(JSONToken::LBRACE);
                }
                '}' => {
                    tokens.push(JSONToken::RBRACE);
                }
                ',' => {
                    tokens.push(JSONToken::COMMA);
                }
                '\"' | '\'' => {
                    tokens.push(JSONToken::QUOTE);
                }
                // literal
                _ => {
                    todo!("unhandled char {:?}", next_char)
                }
            }
        }

        maybe_next_char = characters.next();
    }

    return tokens;
}
