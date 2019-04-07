use regex::Regex;

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    KeyWord(KeyWords), // language Keywords
    Ident(String),     // variable names
    IntNumber(isize),
    Plus,             // +
    Hyphen,           // -
    Asterisk,         // *
    Slash,            // /
    Dot,              // .
    QuestionMark,     // ?
    And,              // &&
    Or,               // ||
    Assign,           // =
    LessThan,         // <
    GreaterThan,      // >
    Equal,            // ==
    NotEqual,         // !=
    LessThanEqual,    // <=
    GreaterThanEqual, // >=
    Colon,            // :
    Tilde,            // ~
    Comma,            // ,
    Delimiter,        // ;
    OpenBracket,      // (
    CloseBracket,     // )
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum KeyWords {
    r#number,
    r#return,
    r#true,
    r#false,
    r#fn,
}

use self::Token::*;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut result = Vec::new();

    let comments = Regex::new(r"(?m)#.*\n").unwrap();

    let no_comments_input = comments.replace_all(input, "\n");

    let tokens_to_match = Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<delimiter>;)|",
        r"(?P<logic>[|&]{2})|",
        r"(?P<bracket>[()])|",
        r"(?P<number>\d+)|",
        r"(?P<inequality><=|==|=|>=|!=|<|>)|",
        r"(?P<operator>\S)"
    ))
    .unwrap();

    for capture in tokens_to_match.captures_iter(no_comments_input.as_ref()) {
        let token = if capture.name("ident").is_some() {
            match capture.name("ident").unwrap().as_str() {
                "number" => KeyWord(KeyWords::r#number),
                "return" => KeyWord(KeyWords::r#return),
                "true" => KeyWord(KeyWords::r#true),
                "false" => KeyWord(KeyWords::r#false),
                "fn" => KeyWord(KeyWords::r#fn),
                ident => Ident(ident.to_string()),
            }
        } else if capture.name("delimiter").is_some() {
            Delimiter
        } else if capture.name("logic").is_some() {
            match capture.name("logic").unwrap().as_str() {
                "&&" => And,
                "||" => Or,
                _ => unimplemented!(),
            }
        } else if capture.name("bracket").is_some() {
            match capture.name("bracket").unwrap().as_str() {
                ")" => CloseBracket,
                "(" => OpenBracket,
                _ => unimplemented!(),
            }
        } else if capture.name("number").is_some() {
            match capture.name("number").unwrap().as_str().parse() {
                Ok(number) => IntNumber(number),
                Err(e) => panic!("Lexer failed trying to parse number : {:?}", e),
            }
        } else if capture.name("inequality").is_some() {
            match capture.name("inequality").unwrap().as_str() {
                "=" => Assign,
                "==" => Equal,
                "<" => LessThan,
                ">" => GreaterThan,
                "!=" => NotEqual,
                "<=" => LessThanEqual,
                ">=" => GreaterThanEqual,
                _ => unreachable!(),
            }
        } else {
            match capture.name("operator").unwrap().as_str() {
                "+" => Plus,
                "-" => Hyphen,
                "*" => Asterisk,
                "/" => Slash,
                _ => unreachable!(),
            }
        };
        result.push(token);
    }
    result
}
