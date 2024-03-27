use std::collections::HashMap;

use crate::expression::BinaryOp;

const INPUT_INT: &str = "IINP";
const INPUT_FLOAT: &str = "RINP";

pub enum CodeReference {
    Literal(String),
    VarName(Box<str>),
}

#[derive(Default)]
pub struct CodeGenerator {
    labels: usize,
    tmp_variables: usize,
    var_types: HashMap<&'static str, VarType>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    Float = 0,
    Int = 1,
}

impl VarType {
    // The type of the result of some binary operation. `self` and `other` are the types of the two operands.
    pub fn combine(self, other: Self) -> Self {
        use VarType::*;
        match (self, other) {
            (Int, Int) => Int,
            _ => Float,
        }
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_var_type(&self, var_name: &str) -> Option<VarType> {
        self.var_types.get(var_name).copied()
    }

    pub fn register_variable(&mut self, var_name: &'static str, ty: VarType) {
        self.var_types.insert(var_name, ty);
    }

    pub fn new_tmp_var(&mut self, ty: VarType) -> CodeReference {
        let tmp_var_name = String::leak(format!("_t{}", self.tmp_variables));
        self.tmp_variables += 1;
        self.register_variable(tmp_var_name, ty);
        CodeReference::VarName(Box::from(&*tmp_var_name))
    }

    // Generated ITOR / RTOI statements
    // This will generate:
    // ITOR a b
    // OR
    // RTOI a b
    pub fn to_stmt(&self, ty: VarType, a: &CodeReference, b: &CodeReference) -> String {
        match ty {
            VarType::Int => format!("RTOI {} {}\n", a, b),
            VarType::Float => format!("ITOR {} {}\n", a, b),
        }
    }

    // Binary operation
    // This will generate:
    // X a b c
    // X in {IADD, ISUB, IDIV, IMLT, RADD, RSUB, RMLT, RDIV}
    pub fn bin_op(
        &self,
        ty: VarType,
        binop: BinaryOp,
        a: &CodeReference,
        b: &CodeReference,
        c: &CodeReference,
    ) -> String {
        let mut op = String::new();

        match ty {
            VarType::Int => {
                op.push_str("I");
            }
            VarType::Float => {
                op.push_str("R");
            }
        }
        match binop {
            BinaryOp::Sub => op.push_str("SUB"),
            BinaryOp::Div => op.push_str("DIV"),
            BinaryOp::Mul => op.push_str("MLT"),
            BinaryOp::Add => op.push_str("ADD"),
        };

        return format!("{} {} {} {}\n", op, a, b, c);
    }

    pub fn gen_input_stmt(&mut self, var_name: Box<str>) -> Option<String> {
        let mut output = String::new();
        let var_type = self.get_var_type(&var_name)?;
        // Use the command for the matching type (INPT / RINP).
        match var_type {
            VarType::Int => output.push_str(INPUT_INT),
            VarType::Float => output.push_str(INPUT_FLOAT),
        }
        // The command takes the variable name as the only argument.
        output.push_str(" ");
        output.push_str(&var_name);
        output.push_str("\n");
        Some(output)
    }
}

impl std::fmt::Display for CodeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeReference::Literal(lit) => write!(f, "{}", lit),
            CodeReference::VarName(var_name) => write!(f, "{}", var_name),
        }
    }
}
