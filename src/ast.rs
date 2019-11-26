use crate::lexer::Token;

pub trait Codegen {
    fn codegen(&self) {}
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExprAST {
    NumberExpr(NumberExprAST),
    VariableExpr(VariableExprAST),
    BinaryExpr(BinaryExprAST),
    CallExpr(CallExprAST),
}

#[derive(PartialEq, Clone, Debug)]
pub struct NumberExprAST {
    pub num: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub struct VariableExprAST {
    pub name: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BinaryExprAST {
    pub op: Token,
    pub lhs: Box<ExprAST>,
    pub rhs: Box<ExprAST>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct CallExprAST {
    pub callee: String,
    pub args: Vec<ExprAST>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PrototypeAST {
    pub name: String,
    pub args: Vec<String>,
}
impl PrototypeAST {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FunctionAST {
    pub prototype: Box<PrototypeAST>,
    pub body: Box<ExprAST>,
}
