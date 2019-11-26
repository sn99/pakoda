use crate::ast::ExprAST::BinaryExpr;
use crate::ast::ExprAST::CallExpr;
use crate::ast::*;
use crate::lexer;
use crate::lexer::KeyWords;
use crate::lexer::Token;

#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    name: String,
    tokens: Vec<Token>,
    cur_token: Token,
    token_position: usize,
}

impl Program {
    fn get_next_token(&mut self) {
        self.cur_token = self.tokens[self.token_position].clone();
        self.token_position += 1;
    }

    fn get_tok_precedence(&mut self) -> isize {
        match self.cur_token {
            Token::Plus => {
                return 20;
            }
            Token::Hyphen => {
                return 20;
            }
            Token::Asterisk => {
                return 40;
            }
            Token::LessThan => {
                return 10;
            }
            _ => {
                return -1;
            }
        }
    }

    pub fn new(file_name: &String, input: &str) -> Self {
        let tokens = lexer::tokenize(input);

        Self {
            name: file_name.to_owned(),
            tokens: lexer::tokenize(input),
            cur_token: tokens[0].clone(),
            token_position: 1,
        }
    }

    pub fn start(&mut self) {
        match self.cur_token {
            Token::Delimiter => self.get_next_token(),
            Token::KeyWord(KeyWords::r#fn) => {
                self.parse_definition();
            }
            Token::KeyWord(KeyWords::r#extern) => {
                self.parse_extern();
            }
            _ => {
                self.handle_top_level_expression();
            }
        }
    }

    fn parse_number_expr(&mut self, num: f64) -> Option<Box<ExprAST>> {
        let result = Box::new(ExprAST::NumberExpr(NumberExprAST { num }));
        self.get_next_token();
        Some(result)
    }

    fn parse_paren_expr(&mut self) -> Option<Box<ExprAST>> {
        self.get_next_token();
        let v = self.parse_expression();

        if v.is_none() {
            return None;
        }

        if self.cur_token != Token::CloseBracket {
            return log_error("expected ')'".to_owned());
        }

        self.get_next_token();

        v
    }

    fn parse_identifier_expr(&mut self, identifier_string: String) -> Option<Box<ExprAST>> {
        let id_name = identifier_string;

        self.get_next_token();

        if self.cur_token != Token::OpenBracket {
            return Some(Box::new(ExprAST::VariableExpr(VariableExprAST {
                name: id_name,
            })));
        }

        self.get_next_token();

        let mut args: Vec<ExprAST> = Vec::new();
        if self.cur_token != Token::CloseBracket {
            loop {
                let e = self.parse_expression();
                match e {
                    Some(arg) => {
                        args.push(*arg);
                    }
                    None => {
                        return None;
                    }
                }

                if self.cur_token == Token::CloseBracket {
                    break;
                }

                if self.cur_token != Token::Comma {
                    return log_error("Expected ')' or ',' in argument list".to_owned());
                }
                self.get_next_token();
            }
        }
        self.get_next_token();

        Some(Box::new(CallExpr(CallExprAST {
            callee: id_name,
            args,
        })))
    }

    pub fn parse_primary(&mut self) -> Option<Box<ExprAST>> {
        let k = self.cur_token.clone();
        match k {
            Token::Ident(e) => {
                return self.parse_identifier_expr(e.clone());
            }
            Token::IntNumber(num) => {
                return self.parse_number_expr(num as f64);
            }
            Token::FloatNumber(num) => {
                return self.parse_number_expr(num);
            }
            Token::OpenCurly => {
                return self.parse_paren_expr();
            }
            _ => {
                log_error("unknown token when expecting an expression".to_owned());
                None
            }
        }
    }

    pub fn parse_expression(&mut self) -> Option<Box<ExprAST>> {
        let lhs = self.parse_primary();

        if lhs.is_none() {
            return None;
        }

        return self.parse_bin_op_rhs(0, lhs);
    }

    pub fn parse_bin_op_rhs(
        &mut self,
        expr_prec: isize,
        mut lhs: Option<Box<ExprAST>>,
    ) -> Option<Box<ExprAST>> {
        loop {
            let tok_prec = self.get_tok_precedence();
            if tok_prec < expr_prec {
                return lhs;
            }

            let bin_op = self.cur_token.clone();
            self.get_next_token();

            let mut rhs = self.parse_primary();
            if rhs.is_none() {
                return None;
            }

            let next_prec = self.get_tok_precedence();

            if tok_prec < next_prec {
                rhs = self.parse_bin_op_rhs(tok_prec + 1, rhs);

                if rhs.is_none() {
                    return None;
                }
            }

            lhs = Some(Box::new(BinaryExpr(BinaryExprAST {
                op: bin_op,
                lhs: lhs.unwrap(),
                rhs: rhs.unwrap(),
            })));
        }
    }

    pub fn parse_prototype(&mut self) -> Option<Box<PrototypeAST>> {
        let fn_name = match &self.cur_token {
            Token::Ident(s) => s.clone(),
            _ => unimplemented!(),
        };
        self.get_next_token();

        if self.cur_token != Token::OpenBracket {
            return log_error_p("Expected '(' in protoype".to_owned());
        }
        let mut arg_names: Vec<String> = Vec::new();
        loop {
            self.get_next_token();
            match &self.cur_token {
                Token::Ident(s) => {
                    arg_names.push(s.clone().to_owned());
                }
                _ => {
                    break;
                }
            }
        }
        if self.cur_token != Token::CloseBracket {
            return log_error_p("Expected ')' in protoype".to_owned());
        }

        self.get_next_token();

        Some(Box::new(PrototypeAST {
            name: fn_name.to_owned(),
            args: arg_names,
        }))
    }

    pub fn parse_definition(&mut self) -> Option<Box<FunctionAST>> {
        self.get_next_token();
        let proto = self.parse_prototype();
        if proto.is_none() {
            return None;
        }

        match self.parse_expression() {
            Some(e) => {
                return Some(Box::new(FunctionAST {
                    prototype: proto.unwrap(),
                    body: e,
                }));
            }
            _ => None,
        }
    }

    pub fn parse_extern(&mut self) -> Option<Box<PrototypeAST>> {
        self.get_next_token();
        self.parse_prototype()
    }

    pub fn parse_top_level_expr(&mut self) -> Option<Box<FunctionAST>> {
        let k = self.parse_expression();
        let q = k.clone();

        match k {
            None => {}
            _ => {
                let proto = Box::new(PrototypeAST {
                    name: "__anon_expr".to_string(),
                    args: vec![],
                });
                return Some(Box::new(FunctionAST {
                    prototype: proto,
                    body: q.unwrap(),
                }));
            }
        }

        None
    }

    pub fn handle_top_level_expression(&mut self) {
        if self.parse_top_level_expr().is_some() {
            println!("Parsed a top-level expr");
        } else {
            self.get_next_token();
        }
    }
}

fn log_error(error: String) -> Option<Box<ExprAST>> {
    eprint!("log_error: {}", error);
    None
}

fn log_error_p(error: String) -> Option<Box<PrototypeAST>> {
    log_error(error);
    None
}
