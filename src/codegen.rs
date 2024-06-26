use crate::{
    boolexpr::{BoolExpr, RelOp},
    error::CodeGenErrorKind,
    expression::{BinaryOp, Expression},
};
use std::collections::HashMap;

const INPUT_INT_COMMAND: &str = "IINP";
const INPUT_FLOAT_COMMAND: &str = "RINP";
const OUTPUT_INT_COMMAND: &str = "IPRT";
const OUTPUT_FLOAT_COMMAND: &str = "RPRT";
const ASSIGN_INT_COMMAND: &str = "IASN";
const ASSIGN_FLOAT_COMMAND: &str = "RASN";
const JUMP_COMMAND: &str = "JUMP";
const JUMP_IF_ZERO_COMMAND: &str = "JMPZ";

/// Reference an expression's result in code. For example:
/// To compile the expression: (1 + 2) * 3
/// The compiler will generate:
/// t0 = 1 + 2  --  the CodeReference of (1 + 2) is t0.
/// t1 = t0 * 3 -- the CodeReference of (1 + 2) * 3 is t1.
#[derive(Clone)]
pub enum CodeReference {
    IntLiteral(i32),
    FloatLiteral(f32),
    VarName(Box<str>),
}

/// Object to keep track of information about generated code. Keep track of the labels, temporary variables, and the types of registered variables.
#[derive(Default)]
pub struct CodeGenerator {
    labels: usize,
    tmp_variables: usize,
    var_types: HashMap<&'static str, VarType>,
}

/// The type of a variable
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    Float,
    Int,
}

/// An object to keep track of a label
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Label {
    id: usize,
}

impl VarType {
    /// The type of the result of some binary operation. `self` and `other` are the types of the two operands.
    /// For example:
    /// <int> + <float> = <float>
    /// <int> + <int> = <int>
    pub fn combine(self, other: Self) -> Self {
        use VarType::*;
        return match (self, other) {
            (Int, Int) => Int,
            _ => Float,
        };
    }

    /// For printing
    pub fn as_str(&self) -> &'static str {
        return match self {
            VarType::Float => "float",
            VarType::Int => "int",
        };
    }
}

impl CodeGenerator {
    pub fn new() -> Self {
        return Self::default();
    }

    /// Get the type of a registered variable.
    pub fn get_var_type(&self, var_name: &str) -> Result<VarType, CodeGenErrorKind> {
        return self
            .var_types
            .get(var_name)
            .copied()
            .ok_or(CodeGenErrorKind::undefined_variable(
                var_name,
                self.var_types
                    .clone()
                    .into_keys()
                    .map(|s| String::from(s))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ));
    }

    /// Register a new variable into the type table.
    pub fn register_variable(&mut self, var_name: &'static str, ty: VarType) {
        self.var_types.insert(var_name, ty);
    }

    /// Create a new temporary variable, it's name will be "_t{id}"
    pub fn new_tmp_var(&mut self, ty: VarType) -> CodeReference {
        let tmp_var_name = String::leak(format!("_t{}", self.tmp_variables));
        self.tmp_variables += 1;
        self.register_variable(tmp_var_name, ty);
        return CodeReference::VarName(Box::from(&*tmp_var_name));
    }

    // Generated ITOR / RTOI statements
    // This will generate:
    // ITOR a b
    // OR
    // RTOI a b
    pub fn gen_cast_stmt(&self, ty: VarType, a: &CodeReference, b: &CodeReference) -> String {
        return match ty {
            VarType::Int => format!("RTOI {} {}\n", a, b),
            VarType::Float => format!("ITOR {} {}\n", a, b),
        };
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

    /// This function will put into `a`:
    /// - 0: if b relop c is false
    /// - >0: if b relop c is true
    pub fn relop(
        &self,
        ty: VarType,
        relop: RelOp,
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

        match relop {
            RelOp::Eq => op.push_str("EQL"),
            RelOp::NotEq => op.push_str("NQL"),
            RelOp::Less => op.push_str("LSS"),
            RelOp::Grt => op.push_str("GRT"),
            _ => unreachable!(),
        }

        return format!("{} {} {} {}\n", op, a, b, c);
    }

    /// Generate an output statement
    pub fn gen_output_stmt(&mut self, expr: Expression) -> Result<String, CodeGenErrorKind> {
        let mut output = String::new();
        // Add the code it took to generate the expression to the output
        output.push_str(&expr.code_generated);
        // Use the command for the matching type (IPRT / RPRT).
        match expr.code_ref {
            CodeReference::FloatLiteral(_) => output.push_str(OUTPUT_FLOAT_COMMAND),
            CodeReference::IntLiteral(_) => output.push_str(OUTPUT_INT_COMMAND),
            CodeReference::VarName(ref var_name) => {
                let var_type = self.get_var_type(var_name)?;
                // Use the command for the matching type (INPT / RINP).
                match var_type {
                    VarType::Int => output.push_str(OUTPUT_INT_COMMAND),
                    VarType::Float => output.push_str(OUTPUT_FLOAT_COMMAND),
                }
            }
        }
        // The command takes the variable name as the only argument.
        output.push_str(&format!(" {}\n", expr.code_ref));
        return Ok(output);
    }

    pub fn gen_input_stmt(&mut self, var_name: &str) -> Result<String, CodeGenErrorKind> {
        let mut output = String::new();
        let var_type = self.get_var_type(&var_name)?;
        // Use the command for the matching type (INPT / RINP).
        match var_type {
            VarType::Int => output.push_str(INPUT_INT_COMMAND),
            VarType::Float => output.push_str(INPUT_FLOAT_COMMAND),
        }
        // The command takes the variable name as the only argument.
        output.push_str(&format!(" {}\n", var_name));
        return Ok(output);
    }

    pub fn gen_assignment_stmt(
        &mut self,
        var_name: &str,
        expr: Expression,
    ) -> Result<String, CodeGenErrorKind> {
        let mut output = String::new();
        let var_type = self.get_var_type(&var_name)?;
        // Return an error if there is a type mismatch
        if expr.ty != var_type {
            return Err(CodeGenErrorKind::type_mismtach(
                CodeReference::VarName(Box::from(var_name)),
                var_type,
                expr.code_ref,
                expr.ty,
            ));
        }
        // Push all the code it tool to generate the expression before the assignment statement
        output.push_str(&expr.code_generated);
        // Use the command for the matching type (IASN / RASN).
        match var_type {
            VarType::Int => output.push_str(ASSIGN_INT_COMMAND),
            VarType::Float => output.push_str(ASSIGN_FLOAT_COMMAND),
        }
        output.push_str(&format!(" {} {}\n", var_name, expr.code_ref));
        return Ok(output);
    }

    /// Register a new label, and return a struct to identify it.
    pub fn new_label(&mut self) -> Label {
        self.labels += 1;
        return Label {
            id: self.labels - 1,
        };
    }

    pub fn gen_label_decleration(&self, label: Label) -> String {
        return format!("L{}:\n", label.id);
    }

    pub fn gen_jump_to_label(&self, label: Label) -> String {
        return format!("{} L{}\n", JUMP_COMMAND, label.id);
    }

    pub fn gen_jump_if_false(&self, label: Label, boolexpr: BoolExpr) -> String {
        return format!(
            "{} L{} {}\n",
            JUMP_IF_ZERO_COMMAND, label.id, boolexpr.code_ref
        );
    }
}

impl std::fmt::Display for CodeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeReference::IntLiteral(lit) => write!(f, "{}", lit),
            CodeReference::FloatLiteral(lit) => write!(f, "{:?}", lit),
            CodeReference::VarName(var_name) => write!(f, "{}", var_name),
        }
    }
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
