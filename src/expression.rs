use crate::{codegen::VarType, lexer::Lexeme, token::Operator};

pub struct Expression {
    ty: VarType,
    expr_tree: Box<Expr>,
}

pub enum Expr {
    IntLit(i32),
    FloatLit(f32),
    Variable { name: Box<str> },
    Cast(Expression, VarType),
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Mul(Expression, Expression),
    Div(Expression, Expression),
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn from_lexeme(l: Lexeme) -> Self {
        match &*l.0 {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Internal Error: Parsing token Lexeme as Binary Operation failed. \n Expected: +, -, *, / \n Found: {}", l.0),
        }
    }
}

impl Expression {
    pub fn cast(cast_type: VarType, expr_to_cast: Expression) -> Self {
        Self {
            ty: cast_type,
            expr_tree: Box::new(Expr::Cast(expr_to_cast, cast_type)),
        }
    }

    pub fn variable(var_name: Box<str>, var_type: VarType) -> Self {
        Self {
            ty: var_type,
            expr_tree: Box::new(Expr::Variable { name: var_name }),
        }
    }

    pub fn int_literal(num: i32) -> Self {
        Self {
            ty: VarType::Int,
            expr_tree: Box::new(Expr::IntLit(num)),
        }
    }

    pub fn float_literal(num: f32) -> Self {
        Self {
            ty: VarType::Float,
            expr_tree: Box::new(Expr::FloatLit(num)),
        }
    }

    pub fn binary_op(expr1: Expression, expr2: Expression, binop: BinaryOp) -> Self {
        let ty = expr1.ty.combine(expr2.ty);
        let expr: Expr = match binop {
            BinaryOp::Add => Expr::Add(expr1, expr2),
            BinaryOp::Sub => Expr::Sub(expr1, expr2),
            BinaryOp::Mul => Expr::Mul(expr1, expr2),
            BinaryOp::Div => Expr::Div(expr1, expr2),
        };

        Expression {
            ty,
            expr_tree: Box::new(expr),
        }
    }
}
