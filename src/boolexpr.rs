use crate::{
    codegen::{CodeGenerator, CodeReference, VarType},
    expression::{BinaryOp, Expression},
    lexer::Lexeme,
};

pub struct BoolExpr {
    pub code_ref: CodeReference,
    pub code_generated: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RelOp {
    Eq,     // ==
    NotEq,  // !=
    Less,   // <
    LessEq, // <=
    Grt,    // >
    GrtEq,  // >=
}

impl RelOp {
    pub fn from_lexeme(l: Lexeme) -> Self {
        match &*l.0 {
            "==" => Self::Eq,
            "!=" => Self::NotEq,
            "<" => Self::Less,
            ">" => Self::Grt,
            "<=" => Self::LessEq,
            ">=" => Self::GrtEq,
            l => panic!("Interal Error: Could not parse lexeme: {l} as RelOp"),
        }
    }
}

impl BoolExpr {
    #[inline]
    pub fn as_expression(self) -> Expression {
        return Expression {
            ty: VarType::Int,
            code_ref: self.code_ref,
            code_generated: self.code_generated,
        };
    }

    #[inline]
    pub fn from_expression(expr: Expression) -> BoolExpr {
        return BoolExpr {
            code_ref: expr.code_ref,
            code_generated: expr.code_generated,
        };
    }

    pub fn not(bool_expr: BoolExpr, codegen: &mut CodeGenerator) -> BoolExpr {
        // A boolean expression is always an Int with value 0 or 1
        // To get Not(bool) we do 1 - bool
        // not(false) = 1 - 0 = 1 = true
        // not(true) = 1 - 1 = 0 = false
        return BoolExpr::from_expression(Expression::binary_op(
            Expression::int_literal(1),
            bool_expr.as_expression(),
            BinaryOp::Sub,
            codegen,
        ));
    }

    pub fn and(
        bool_expr1: BoolExpr,
        bool_expr2: BoolExpr,
        codegen: &mut CodeGenerator,
    ) -> BoolExpr {
        // A boolean expression is always an Int with value 0 or 1
        // To get a AND b we do a * b
        // false AND false = 0 * 0 = 0
        // false AND true = 0 * 1 = 0
        // true AND false = 1 * 0 = 0
        // true AND true = 1 * 1 = 1
        return BoolExpr::from_expression(Expression::binary_op(
            bool_expr1.as_expression(),
            bool_expr2.as_expression(),
            BinaryOp::Mul,
            codegen,
        ));
    }

    pub fn or(bool_expr1: BoolExpr, bool_expr2: BoolExpr, codegen: &mut CodeGenerator) -> BoolExpr {
        // A boolean expression is always an Int with value 0 or 1
        // To get a OR b we do: not(not(a) AND not(b)) = 1 - ((1 - a) * (1 - b))
        // false OR false = not(not(false) AND not(false)) = 1 - (1 * 1) = 0 = false
        // false OR true = not(not(false) AND not(true)) = 1 - (1 * 0) = 1 = true
        // true OR false = not(not(true) AND not(false)) = 1 - (0 * 1) = 1 = true
        // true OR true = not(not(true) AND not(true)) = 1 - (0 * 0) = 1 = true
        return BoolExpr::not(
            BoolExpr::and(
                BoolExpr::not(bool_expr1, codegen),
                BoolExpr::not(bool_expr2, codegen),
                codegen,
            ),
            codegen,
        );
    }

    pub fn relop(
        mut expr1: Expression,
        mut expr2: Expression,
        relop: RelOp,
        codegen: &mut CodeGenerator,
    ) -> BoolExpr {
        let code_ref = codegen.new_tmp_var(VarType::Int);
        let expr_ty = expr1.ty.combine(expr2.ty);
        match (expr1.ty, expr2.ty) {
            (VarType::Float, VarType::Int) => {
                expr2 = Expression::cast(VarType::Float, expr2, codegen);
            }
            (VarType::Int, VarType::Float) => {
                expr1 = Expression::cast(VarType::Float, expr1, codegen);
            }
            _ => {}
        }

        let code_generated: String;
        match relop {
            RelOp::GrtEq => {
                let less = BoolExpr::relop(expr1, expr2, RelOp::Less, codegen);
                return BoolExpr::not(less, codegen);
            }
            RelOp::LessEq => {
                // <= is not supported in QUAD, so we turn <= into NOT ( > )
                let greater = BoolExpr::relop(expr1, expr2, RelOp::Grt, codegen);
                return BoolExpr::not(greater, codegen);
            }
            relop => {
                code_generated = format!(
                    "{}{}{}",
                    expr1.code_generated,
                    expr2.code_generated,
                    codegen.relop(expr_ty, relop, &code_ref, &expr1.code_ref, &expr2.code_ref)
                );
            }
        };

        return BoolExpr {
            code_ref,
            code_generated,
        };
    }
}
