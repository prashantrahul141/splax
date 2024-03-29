use crate::{parser::error::ParserError, token::token_main::Token};

use super::expr_ast::Expr;

/// Top level statements enum.
#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Box<StmtBlock>),
    Expr(Box<StmtExpr>),
    Function(Box<StmtFunc>),
    If(Box<StmtIf>),
    Print(Box<StmtPrint>),
    Let(Box<StmtLet>),
    While(Box<StmtWhile>),
}

/// Visitor trait for statements.
/// Since we will be using visitor pattern for the statements.
///
/// When any new pass/feature we need to implement to the statements,
/// we just impl this visitor trait to that struct.
pub trait StmtVisitor {
    fn visit_block_stmt(&mut self, stmt: &StmtBlock);
    fn visit_expression_stmt(&mut self, stmt: &StmtExpr);
    fn visit_print_stmt(&mut self, stmt: &StmtPrint);
    fn visit_let_stmt(&mut self, stmt: &StmtLet);
    fn visit_if_stmt(&mut self, stmt: &StmtIf);
    fn visit_while_stmt(&mut self, stmt: &StmtWhile);
    fn visit_function_stmt(&mut self, stmt: &StmtFunc);
}

/// Walker, in other implementation this will be called `accept`.
/// # Arguments
/// * `visitor` - The visitor struct which implements StmtVisitor trait.
/// * `stmt` - The stmt to walk.
pub fn walk_stmt(visitor: &mut dyn StmtVisitor, stmt: &Stmt) {
    match stmt {
        Stmt::Block(stmt) => visitor.visit_block_stmt(stmt),
        Stmt::Expr(stmt) => visitor.visit_expression_stmt(stmt),
        Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
        Stmt::Let(stmt) => visitor.visit_let_stmt(stmt),
        Stmt::If(stmt) => visitor.visit_if_stmt(stmt),
        Stmt::While(stmt) => visitor.visit_while_stmt(stmt),
        Stmt::Function(stmt) => visitor.visit_function_stmt(stmt),
    }
}

/// Grammer for stmtblock statemments.
#[derive(Debug, Clone)]
pub struct StmtBlock {
    // the expression itself.
    pub block_statements: Vec<Stmt>,
}

/// Grammer for stmtexpr statemments.
#[derive(Debug, Clone)]
pub struct StmtExpr {
    // the expression itself.
    pub expr: Expr,
}

/// Grammer for stmtif statemments.
#[derive(Debug, Clone)]
pub struct StmtIf {
    // condition of if statement.
    pub condition: Expr,
    // then branch of if statement.
    pub then_branch: Stmt,
    // else branch of if statement.
    pub else_branch: Result<Stmt, ParserError>,
}

/// Grammer for stmtprint statemments.
#[derive(Debug, Clone)]
pub struct StmtPrint {
    // the expression to evaluate and print.
    pub expr: Expr,
}

/// Grammer for stmtlet statemments.
#[derive(Debug, Clone)]
pub struct StmtLet {
    // name of the binding.
    pub name: Token,
    // init expr value of the binding.
    pub initialiser: Expr,
}

/// Grammer for stmtwhile statemments.
#[derive(Debug, Clone)]
pub struct StmtWhile {
    // condition of while statement.
    pub condition: Expr,
    // then branch of while statement.
    pub body: Stmt,
}

/// Grammer for function declaration.
#[derive(Debug, Clone)]
pub struct StmtFunc {
    // name of the function.
    pub name: Token,
    // parameters of the function
    pub params: Vec<Token>,
    // function body
    pub body: StmtBlock,
}
