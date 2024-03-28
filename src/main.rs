#![allow(dead_code)]
mod boolexpr;
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
    fn test_expressions() {
        compilation_test_template(
            "2 + 2 * 3 + 1",
            "IMLT _t0 2 3\nIADD _t1 _t0 1\nIADD _t2 2 _t1\n",
            compile_expression,
        );
        compilation_test_template(
            "2.0 + 2 * 3 + 1",
            "IMLT _t0 2 3\nIADD _t1 _t0 1\nITOR _t2 _t1\nRADD _t3 2 _t2\n",
            compile_expression,
        );
        compilation_test_template(
            "((2.0 + 2)) * 3 + 1.0",
            "ITOR _t0 2\nRADD _t1 2 _t0\nITOR _t2 3\nRMLT _t3 _t1 _t2\nRADD _t4 _t3 1\n",
            compile_expression,
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

    #[test]
    fn test_bool_expression() {
        compilation_test_template(
            "1 + 1 > 3 || 2 == 1 + 1",
            "IADD _t0 1 1\n\
            IGRT _t1 _t0 3\n\
            ISUB _t4 1 _t1\n\
            IADD _t2 1 1\n\
            IEQL _t3 2 _t2\n\
            ISUB _t5 1 _t3\n\
            IMLT _t6 _t4 _t5\n\
            ISUB _t7 1 _t6\n",
            compile_bool_expression,
        );
    }

    #[should_panic(expected = "Undeclared Variable")]
    #[test]
    fn test_error_1() {
        compile_expression("var + var");
    }

    #[should_panic(expected = "Unexpected Token")]
    #[test]
    fn test_error_2() {
        compile_expression("while");
    }

    #[should_panic(expected = "Unexpected EOF")]
    #[test]
    fn test_error_3() {
        compile_bool_expression("1 + 1");
    }

    fn compilation_test_template(
        to_compile: &str,
        expected: &str,
        compiler: impl Fn(&str) -> String,
    ) {
        let compiled = compiler(to_compile);
        if expected != compiled {
            panic!(
                "\n\nUnexpected result while compiling: {}.\n\nExpected: \n{}\n\nFound: \n{}",
                to_compile, expected, compiled
            );
        }
    }

    fn compile_expression(expr: &str) -> String {
        Parser::new(Lexer::lex_tokens(String::from(expr)))
            .parse_expression()
            .unwrap()
            .code_generated
    }

    fn compile_bool_expression(expr: &str) -> String {
        Parser::new(Lexer::lex_tokens(String::from(expr)))
            .parse_boolexpr()
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

    fn compile_bool_expression_with_variables(expr: &str, vars: &[(String, VarType)]) -> String {
        let mut parser = Parser::new(Lexer::lex_tokens(String::from(expr)));
        for (var_name, var_type) in vars.into_iter() {
            parser
                .code_generator
                .register_variable(String::leak(var_name.clone()), *var_type)
        }
        parser.parse_boolexpr().unwrap().code_generated
    }
}
