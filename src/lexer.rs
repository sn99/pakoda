use regex::Regex;

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    Add,    // +
    Hyphen, // -
    KeyWord(KeyWords),
    Asterisk,     // *
    Slash,        // /
    Dot,          // .
    QuestionMark, // ?
    And,          // &&
    Or,           // ||
    Equal,        // ==
    NotEqual,     // !=
    LessEqual,    // <=
    GreaterEqual, // >=
    Colon,        // :
    Tilde,        // ~
    Assign,       // =
    Comma,        // ,
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum KeyWords {
    r#int,
    r#return,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    result
}
