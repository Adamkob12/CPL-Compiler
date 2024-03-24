use std::collections::HashMap;

#[derive(Default)]
struct CodeGenerator<'a> {
    generated_code: String,
    labels: usize,
    variables: HashMap<&'a str, Variable>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Type {
    Float = 0,
    Int = 1,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Variable {
    ty: Type,
}

impl<'a> CodeGenerator<'a> {
    pub fn init() -> Self {
        Self::default()
    }
}
