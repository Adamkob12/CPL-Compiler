use crate::{
    codegen::{CodeGenerator, CodeReference, VarType},
    lexer::Lexeme,
};

pub struct Expression {
    ty: VarType,
    expr_tree: Box<Expr>,
    code_ref: CodeReference,
    pub code_generated: String,
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub fn cast(
        cast_type: VarType,
        mut expr_to_cast: Expression,
        codegen: &mut CodeGenerator,
    ) -> Self {
        let var_name = codegen.new_tmp_var(cast_type);
        let mut code_generated = std::mem::take(&mut expr_to_cast.code_generated);
        code_generated.push_str(&codegen.to_stmt(cast_type, &var_name, &expr_to_cast.code_ref));

        Self {
            ty: cast_type,
            expr_tree: Box::new(Expr::Cast(expr_to_cast, cast_type)),
            code_ref: var_name,
            code_generated,
        }
    }

    pub fn variable(var_name: Box<str>, var_type: VarType) -> Self {
        Self {
            ty: var_type,
            expr_tree: Box::new(Expr::Variable {
                name: var_name.clone(),
            }),
            code_ref: CodeReference::VarName(var_name),
            code_generated: String::new(),
        }
    }

    pub fn int_literal(num: i32) -> Self {
        Self {
            ty: VarType::Int,
            expr_tree: Box::new(Expr::IntLit(num)),
            code_ref: CodeReference::Literal(format!("{}", num)),
            code_generated: String::new(),
        }
    }

    pub fn float_literal(num: f32) -> Self {
        Self {
            ty: VarType::Float,
            expr_tree: Box::new(Expr::FloatLit(num)),
            code_ref: CodeReference::Literal(format!("{}", num)),
            code_generated: String::new(),
        }
    }

    pub fn binary_op(
        mut expr1: Expression,
        mut expr2: Expression,
        binop: BinaryOp,
        codegen: &mut CodeGenerator,
    ) -> Self {
        let ty = expr1.ty.combine(expr2.ty);
        match (expr1.ty, expr2.ty) {
            (VarType::Float, VarType::Int) => {
                expr2 = Expression::cast(VarType::Float, expr2, codegen);
            }
            (VarType::Int, VarType::Float) => {
                expr1 = Expression::cast(VarType::Float, expr1, codegen);
            }
            _ => {}
        }
        let tmp_var = codegen.new_tmp_var(ty);
        let code_generated = format!(
            "{}{}{}",
            std::mem::take(&mut expr1.code_generated),
            std::mem::take(&mut expr2.code_generated),
            codegen.bin_op(ty, binop, &tmp_var, &expr1.code_ref, &expr2.code_ref)
        );
        let expr: Expr = match binop {
            BinaryOp::Add => Expr::Add(expr1, expr2),
            BinaryOp::Sub => Expr::Sub(expr1, expr2),
            BinaryOp::Mul => Expr::Mul(expr1, expr2),
            BinaryOp::Div => Expr::Div(expr1, expr2),
        };
        Expression {
            ty,
            expr_tree: Box::new(expr),
            code_ref: tmp_var,
            code_generated,
        }
    }
}
