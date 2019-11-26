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
    curToken: Token,
    token_position: usize,
}

impl Program {
    fn getNextToken(&mut self) {
        self.curToken = self.tokens[self.token_position].clone();
        self.token_position += 1;
    }

    fn GetTokPrecedence(&mut self) -> isize {
        match self.curToken {
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
        -1
    }

    pub fn new(file_name: &String, input: &str) -> Self {
        let tokens = lexer::tokenize(input);

        Self {
            name: file_name.to_owned(),
            tokens: lexer::tokenize(input),
            curToken: tokens[0].clone(),
            token_position: 1,
        }
    }

    pub fn start(&mut self) {
        match self.curToken {
            Token::Delimiter => self.getNextToken(),
            Token::KeyWord(KeyWords::r#fn) => {
                self.ParseDefinition();
            }
            Token::KeyWord(KeyWords::r#extern) => {
                self.ParseExtern();
            }
            _ => {
                self.HandleTopLevelExpression();
            }
        }
    }

    fn ParseNumberExpr(&mut self, num: f64) -> Option<Box<ExprAST>> {
        let result = Box::new(ExprAST::NumberExpr(NumberExprAST { num }));
        self.getNextToken();
        Some(result)
    }

    fn ParseParenExpr(&mut self) -> Option<Box<ExprAST>> {
        self.getNextToken();
        let V = self.ParseExpression();

        if V.is_none() {
            return None;
        }

        if self.curToken != Token::CloseBracket {
            return LogError("expected ')'".to_owned());
        }

        self.getNextToken();

        V
    }

    fn ParseIdentifierExpr(&mut self, IdentifierString: String) -> Option<Box<ExprAST>> {
        let IdName = IdentifierString;

        self.getNextToken();

        if self.curToken != Token::OpenBracket {
            return Some(Box::new(ExprAST::VariableExpr(VariableExprAST {
                name: IdName,
            })));
        }

        self.getNextToken();

        let mut args: Vec<ExprAST> = Vec::new();
        if self.curToken != Token::CloseBracket {
            loop {
                let e = self.ParseExpression();
                match e {
                    Some(arg) => {
                        args.push(*arg);
                    }
                    None => {
                        return None;
                    }
                }

                if self.curToken == Token::CloseBracket {
                    break;
                }

                if self.curToken != Token::Comma {
                    return LogError("Expected ')' or ',' in argument list".to_owned());
                }
                self.getNextToken();
            }
        }
        self.getNextToken();

        Some(Box::new(CallExpr(CallExprAST {
            callee: IdName,
            args,
        })))
    }

    pub fn ParsePrimary(&mut self) -> Option<Box<ExprAST>> {
        match &self.curToken {
            Token::Ident(e) => {
                return self.ParseIdentifierExpr(e.clone());
            }
            Token::IntNumber(num) => {
                return self.ParseNumberExpr(*num as f64);
            }
            Token::FloatNumber(num) => {
                return self.ParseNumberExpr(*num);
            }
            Token::OpenCurly => {
                return self.ParseParenExpr();
            }
            _ => {
                LogError("unknown token when expecting an expression".to_owned());
                None
            }
        }
    }

    pub fn ParseExpression(&mut self) -> Option<Box<ExprAST>> {
        let lhs = self.ParsePrimary();

        if lhs.is_none() {
            return None;
        }

        return self.ParseBinOpRHS(0, lhs);
    }

    pub fn ParseBinOpRHS(
        &mut self,
        ExprPrec: isize,
        mut lhs: Option<Box<ExprAST>>,
    ) -> Option<Box<ExprAST>> {
        loop {
            let TokPrec = self.GetTokPrecedence();
            if TokPrec < ExprPrec {
                return lhs;
            }

            let BinOp = self.curToken.clone();
            self.getNextToken();

            let mut rhs = self.ParsePrimary();
            if rhs.is_none() {
                return None;
            }

            let NextPrec = self.GetTokPrecedence();

            if TokPrec < NextPrec {
                rhs = self.ParseBinOpRHS(TokPrec + 1, rhs);

                if rhs.is_none() {
                    return None;
                }
            }

            lhs = Some(Box::new(BinaryExpr(BinaryExprAST {
                op: BinOp,
                lhs: lhs.unwrap(),
                rhs: rhs.unwrap(),
            })));
        }
    }

    pub fn ParsePrototype(&mut self) -> Option<Box<PrototypeAST>> {
        let FnName = match &self.curToken {
            Token::Ident(s) => s.clone(),
            _ => unimplemented!(),
        };
        self.getNextToken();

        if self.curToken != Token::OpenBracket {
            return LogErrorP("Expected '(' in protoype".to_owned());
        }
        let mut ArgNames: Vec<String> = Vec::new();
        loop {
            self.getNextToken();
            match &self.curToken {
                Token::Ident(s) => {
                    ArgNames.push(s.clone().to_owned());
                }
                _ => {
                    break;
                }
            }
        }
        if self.curToken != Token::CloseBracket {
            return LogErrorP("Expected ')' in protoype".to_owned());
        }

        self.getNextToken();

        Some(Box::new(PrototypeAST {
            name: FnName.to_owned(),
            args: ArgNames,
        }))
    }

    pub fn ParseDefinition(&mut self) -> Option<Box<FunctionAST>> {
        self.getNextToken();
        let Proto = self.ParsePrototype();
        if Proto.is_none() {
            return None;
        }

        match self.ParseExpression() {
            Some(e) => {
                return Some(Box::new(FunctionAST {
                    prototype: Proto.unwrap(),
                    body: e,
                }));
            }
            _ => unimplemented!(),
        }

        None
    }

    pub fn ParseExtern(&mut self) -> Option<Box<PrototypeAST>> {
        self.getNextToken();
        self.ParsePrototype()
    }

    pub fn ParseTopLevelExpr(&mut self) -> Option<Box<FunctionAST>> {
        let k = self.ParseExpression();
        let q = k.clone();

        match k {
            None => {}
            _ => {
                let Proto = Box::new(PrototypeAST {
                    name: "__anon_expr".to_string(),
                    args: vec![],
                });
                return Some(Box::new(FunctionAST {
                    prototype: Proto,
                    body: q.unwrap(),
                }));
            }
        }

        None
    }

    pub fn HandleTopLevelExpression(&mut self) {
        if self.ParseTopLevelExpr().is_some() {
            println!("Parsed a top-level expr");
        } else {
            self.getNextToken();
        }
    }
}

fn LogError(error: String) -> Option<Box<ExprAST>> {
    eprint!("LogError: {}", error);
    None
}

fn LogErrorP(error: String) -> Option<Box<PrototypeAST>> {
    LogError(error);
    None
}
