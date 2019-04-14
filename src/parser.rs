/*
*/

use crate::lexer;
use crate::lexer::{KeyWords, Token};

type Statement = Vec<Expression>;

#[derive(PartialEq, Clone, Debug)]
enum DataType {
    Number(Option<isize>),
}

#[derive(PartialEq, Clone, Debug)]
enum Expression {
    DataType(String, Option<DataType>),
    Operator(Operator),
    FunctionNode(Function),
    PrototypeNode(Prototype),
    InequalityNode(Inequality),
    LogicalNode(LogicalOperator),
}

#[derive(PartialEq, Clone, Debug)]
struct ASTNode {
    function: Function,
}

#[derive(PartialEq, Clone, Debug)]
struct Function {
    prototype: Prototype,
    r#return: Option<Box<Expression>>,
    body: Option<Statement>,
}

#[derive(PartialEq, Clone, Debug)]
struct Prototype {
    name: String,
    args: Option<Vec<Expression>>,
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
pub struct Program {
    name: String,
    node: Vec<ASTNode>,
    tokens: Vec<Token>,
    token_position: usize,
}

impl Program {
    pub fn new(file_name: &String, input: &str) -> Self {
        let tokens = lexer::tokenize(input);

        Self {
            name: file_name.to_owned(),
            node: Vec::new(),
            tokens,
            token_position: 0,
        }
    }

    pub fn parse(&mut self) {
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
        self.token_position += 1;
        let name = self.tokens[self.token_position].clone().get_string();

        self.token_position += 1;
        if self.tokens[self.token_position] != Token::OpenBracket {
            panic!("Expected '(' at {}", self.token_position);
        }

        let mut args: Option<Vec<Expression>> = None;
        let mut g = Vec::new();
        self.token_position += 1;
        if self.tokens[self.token_position] != Token::CloseBracket {
            args = Some(Vec::new());
            while self.tokens[self.token_position] != Token::CloseBracket {
                if self.tokens[self.token_position] == Token::Comma {
                    self.token_position += 1;
                }

                if self.tokens[self.token_position] != Token::KeyWord(KeyWords::number) {
                    panic!("Expected parameter type like 'number'");
                }

                self.token_position += 1;

                g.push(Expression::DataType(
                    self.tokens[self.token_position]
                        .clone()
                        .get_string()
                        .to_owned(),
                    None,
                ));
                self.token_position += 1;
            }
        }

        if g.len() != 0 {
            args = Some(g);
        }

        self.token_position += 1;
        if self.tokens[self.token_position] != Token::OpenCurly {
            panic!("Expected '{}' at position {}", "{", self.token_position);
        }
        self.token_position += 1;
        let mut body = None;
        let mut r#return: Option<Box<Expression>> = None;
        if self.tokens[self.token_position] == Token::KeyWord(KeyWords::r#return) {
            self.token_position += 1;
            if let Token::IntNumber(value) = self.tokens[self.token_position] {
                r#return = Some(Box::new(Expression::DataType(
                    "Number".to_owned(),
                    Some(DataType::Number(Some(value))),
                )));
            }
        }

        self.token_position += 1;
        if self.tokens[self.token_position] != Token::Delimiter {
            panic!("Expected ';'");
        }
        self.token_position += 1;
        if self.tokens[self.token_position] != Token::CLoseCurly {
            panic!("Expected '{}'", "}");
        }
        self.token_position += 1;

        let function = Function {
            prototype: Prototype { name, args },
            r#return,
            body,
        };

        self.node.push(ASTNode { function });
    }
}
