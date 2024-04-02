use crate::{
    codegen::{CodeGenerator, CodeReference, VarType},
    lexer::Lexeme,
};

/// An object with the information needed to parse an expression.
/// ty: The type of the expression (Int / Float)
/// code_ref: The way to reference this expression in code (variable name / int literal / float literal)
/// code_generated: The code it took to generate this expression so far
pub struct Expression {
    pub ty: VarType,
    pub code_ref: CodeReference,
    pub code_generated: String,
}

/// Binary operation between two expression
/// Expression BinaryOp Expression
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

impl BinaryOp {
    pub fn from_lexeme(l: Lexeme) -> Self {
        return match &*l.0 {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Internal Error: Parsing token Lexeme as Binary Operation failed. \n Expected: +, -, *, / \n Found: {}", l.0),
        };
    }
}

impl Expression {
    /// Cast this expression as another type
    pub fn cast(cast_type: VarType, expr_to_cast: Expression, codegen: &mut CodeGenerator) -> Self {
        let var_name = codegen.new_tmp_var(cast_type);
        let mut code_generated = expr_to_cast.code_generated;
        code_generated.push_str(&codegen.gen_cast_stmt(
            cast_type,
            &var_name,
            &expr_to_cast.code_ref,
        ));

        return Self {
            ty: cast_type,
            code_ref: var_name,
            code_generated,
        };
    }

    /// An expression that is just a variable
    pub fn variable(var_name: Box<str>, var_type: VarType) -> Self {
        return Self {
            ty: var_type,
            code_ref: CodeReference::VarName(var_name),
            code_generated: String::new(),
        };
    }

    /// An expression that is just an integer literal
    pub fn int_literal(num: i32) -> Self {
        return Self {
            ty: VarType::Int,
            code_ref: CodeReference::IntLiteral(num),
            code_generated: String::new(),
        };
    }

    /// An expression that is just an float literal
    pub fn float_literal(num: f32) -> Self {
        return Self {
            ty: VarType::Float,
            code_ref: CodeReference::FloatLiteral(num),
            code_generated: String::new(),
        };
    }

    /// A binary operation between two expressions
    pub fn binary_op(
        mut expr1: Expression,
        mut expr2: Expression,
        binop: BinaryOp,
        codegen: &mut CodeGenerator,
    ) -> Self {
        let ty = expr1.ty.combine(expr2.ty);
        // Infer the type of the resulting expression, if needed, cast the expression to a different type.
        match (expr1.ty, expr2.ty) {
            (VarType::Float, VarType::Int) => {
                expr2 = Expression::cast(VarType::Float, expr2, codegen);
            }
            (VarType::Int, VarType::Float) => {
                expr1 = Expression::cast(VarType::Float, expr1, codegen);
            }
            _ => {}
        }
        // Inherit the code generated of the other expressions.
        let tmp_var = codegen.new_tmp_var(ty);
        let code_generated = format!(
            "{}{}{}",
            expr1.code_generated,
            expr2.code_generated,
            codegen.bin_op(ty, binop, &tmp_var, &expr1.code_ref, &expr2.code_ref)
        );
        return Expression {
            ty,
            code_ref: tmp_var,
            code_generated,
        };
    }
}
