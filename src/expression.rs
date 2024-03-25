use crate::codegen::VarType;

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
}
