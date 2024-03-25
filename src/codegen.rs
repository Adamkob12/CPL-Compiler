use std::collections::HashMap;

const INPUT_INT: &str = "IINP";
const INPUT_FLOAT: &str = "RINP";

#[derive(Default)]
pub struct CodeGenerator {
    labels: usize,
    tmp_variables: usize,
    var_types: HashMap<Box<str>, VarType>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    Float = 0,
    Int = 1,
}

impl CodeGenerator {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn get_var_type(&self, var_name: &Box<str>) -> Option<VarType> {
        self.var_types.get(var_name).copied()
    }
}

impl CodeGenerator {
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
        Some(output)
    }
}
