use super::expr_ast::ExprVisitor;
use crate::ast::expr_ast::walk_expr;

// to print the ast of parsed expressions.
pub struct AstPrinter;

impl AstPrinter {
    #[allow(dead_code)]
    // this is a debug only struct, we might not want to always create the
    // AST tree string representation, which can result in poor performance,
    // so disable dead code lint here.
    pub fn new() -> Self {
        Self
    }
}

// implementing visitor for AstPrinter.
impl ExprVisitor<()> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &super::expr_ast::ExprBinary) {
        print!("{} {} {}", expr.operator.lexeme, expr.left, expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &super::expr_ast::ExprGrouping) {
        print!("( group");
        walk_expr(self, &expr.expression);
        print!(")");
    }

    fn visit_literal_expr(&mut self, expr: &super::expr_ast::ExprLiteral) {
        print!(" {} ", expr.value);
    }

    fn visit_unary_expr(&mut self, expr: &super::expr_ast::ExprUnary) {
        print!("( {}", expr.operator.lexeme);
        walk_expr(self, &expr.right);
        print!(")");
    }

    fn visit_let_expr(&mut self, expr: &super::expr_ast::ExprVariable) {
        print!("let {}", expr.name);
    }

    fn visit_assign_expr(&mut self, expr: &super::expr_ast::ExprAssign) {
        print!("= {} {}", expr.name, expr.value);
    }

    fn visit_logical_expr(&mut self, expr: &super::expr_ast::ExprLogical) {
        print!("{} {} {}", expr.operator.lexeme, expr.left, expr.right);
    }

    fn visit_call_expr(&mut self, expr: &super::expr_ast::ExprCall) {
        print!("{}", expr.callee);
    }
}
