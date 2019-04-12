/*
*/
use std::collections::HashMap;

use crate::lexer;
use crate::lexer::{KeyWords, Token};

type Statement = Vec<Expression>;

#[derive(PartialEq, Clone, Debug)]
enum DataType {
    Number(isize),
}

#[derive(PartialEq, Clone, Debug)]
enum Expression {
    VariableExpr(String),
    DataType(DataType),
    Operator(Operator),
    FunctionNode(Function),
    InequalityNode(Inequality),
    LogicalNode(LogicalOperator),
}

#[derive(PartialEq, Clone, Debug)]
struct ASTNode {
    function: Function,
}

#[derive(PartialEq, Clone, Debug)]
struct Function {
    name: String,
    args: Option<Vec<HashMap<Option<String>, Expression>>>,
    r#return: Option<Box<Expression>>,
    body: Option<Statement>,
}

#[derive(PartialEq, Clone, Debug)]
struct Prototype {
    name: String,
    args: Option<Vec<HashMap<Option<String>, Expression>>>,
}

#[derive(PartialEq, Clone, Debug)]
struct Operator {
    operator: String,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(PartialEq, Clone, Debug)]
struct Inequality {
    inequality: String,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(PartialEq, Clone, Debug)]
struct LogicalOperator {
    operator: String,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(PartialEq, Clone, Debug)]
struct Program {
    name: String,
    node: Vec<ASTNode>,
    tokens: Vec<Token>,
    token_position: usize,
}

impl Program {
    fn new(file_name: String, input: &str) -> Self {
        let tokens = lexer::tokenize(input);

        Self {
            name: file_name,
            node: Vec::new(),
            tokens,
            token_position: 0,
        }
    }

    fn parse(&mut self) {
        if self.token_position == 0
            && self.tokens[self.token_position] != Token::KeyWord(KeyWords::r#fn)
        {
            panic!("Start with a function !!!");
        } else {
            loop {
                if self.token_position >= self.tokens.len() {
                    break;
                }
                match self.tokens[self.token_position] {
                    Token::KeyWord(KeyWords::r#fn) => self.parse_function(),
                    _ => unimplemented!(),
                }
            }
        }
    }

    fn parse_function(&mut self) {
        let mut simple_bracket_count = 0;
        loop {
            self.token_position += 1;
            let mut current_token = self.tokens[self.token_position].clone();
            if let Token::Ident(name) = current_token {
                self.token_position += 1;
                if self.tokens[self.token_position].clone() == Token::OpenBracket {
                    simple_bracket_count += 1;
                    while self.tokens[self.token_position].clone() != Token::CloseBracket {
                        self.token_position += 1;
                    }
                }
            } else {
                panic!("Expected name at position {}",)
            }
        }
    }
}
