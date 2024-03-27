#![allow(dead_code)]
mod codegen;
mod compiler;
mod expression;
mod lexer;
mod parser;
mod token;

fn main() {}

// TESTS

#[cfg(test)]
mod tests {
    use super::parser::Parser;
    use crate::{codegen::VarType, lexer::Lexer};

    #[test]
    fn test_compile_expressions() {
        assert_eq!(
            "IMLT _t0 2 3\nIADD _t1 _t0 1\nIADD _t2 2 _t1\n",
            compile_expression("2 + 2 * 3 + 1")
        );
        assert_eq!(
            "IMLT _t0 2 3\nIADD _t1 _t0 1\nITOR _t2 _t1\nRADD _t3 2 _t2\n",
            compile_expression("2.0 + 2 * 3 + 1")
        );
        assert_eq!(
            "ITOR _t0 2\nRADD _t1 2 _t0\nITOR _t2 3\nRMLT _t3 _t1 _t2\nRADD _t4 _t3 1\n",
            compile_expression("((2.0 + 2)) * 3 + 1.0"),
        );
        assert_eq!(
            "ITOR _t0 var2\nRADD _t1 var _t0\nRMLT _t2 var _t1\n",
            compile_expression_with_variables(
                "var * (var + static_cast<float> (var2))",
                &[
                    (String::from("var"), VarType::Float),
                    (String::from("var2"), VarType::Int)
                ]
            )
        );
    }

    fn compile_expression(expr: &str) -> String {
        Parser::new(Lexer::lex_tokens(String::from(expr)))
            .parse_expression()
            .unwrap()
            .code_generated
    }

    fn compile_expression_with_variables(expr: &str, vars: &[(String, VarType)]) -> String {
        let mut parser = Parser::new(Lexer::lex_tokens(String::from(expr)));
        for (var_name, var_type) in vars.into_iter() {
            parser
                .code_generator
                .register_variable(String::leak(var_name.clone()), *var_type)
        }
        parser.parse_expression().unwrap().code_generated
    }
}
