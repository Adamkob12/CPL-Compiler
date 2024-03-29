use crate::{
    codegen::{CodeReference, VarType},
    token::Token,
};
use std::fmt::Display;

pub enum CodeGenErrorKind {
    UndeclaredVariable {
        varname: String,
        all_variables: Box<[String]>,
    },
    TypeMismatchInAssignment {
        expected_ref: CodeReference,
        expected_type: VarType,
        found_ref: CodeReference,
        found_type: VarType,
    },
}

pub enum ParsingErrorKind {
    UnexpectedEOF,
    UnexpectedToken {
        expected: Box<[Token]>,
        found: Token,
    },
}

pub enum LexingErrorKind {}

pub struct CompilationError {
    line: usize,
    column: usize,
    err_kind: CompilationErrorKind,
}

pub enum CompilationErrorKind {
    InternalError(String),
    ParsingError(ParsingErrorKind),
    CodeGenError(CodeGenErrorKind),
    LexingError(LexingErrorKind),
}

impl CompilationError {
    pub fn parsing_error(line: usize, column: usize, err_kind: ParsingErrorKind) -> Self {
        return Self {
            line,
            column,
            err_kind: CompilationErrorKind::ParsingError(err_kind),
        };
    }

    pub fn codegen_error(line: usize, column: usize, err_kind: CodeGenErrorKind) -> Self {
        return Self {
            line,
            column,
            err_kind: CompilationErrorKind::CodeGenError(err_kind),
        };
    }

    pub fn internal_error(desc: String) -> Self {
        return Self {
            line: 0,
            column: 0,
            err_kind: CompilationErrorKind::InternalError(desc),
        };
    }

    pub fn unexpected_eof() -> Self {
        return Self {
            line: 0,
            column: 0,
            err_kind: CompilationErrorKind::ParsingError(ParsingErrorKind::UnexpectedEOF),
        };
    }
}

impl CodeGenErrorKind {
    pub fn undefined_variable(var_name: &str, all_variables: Box<[String]>) -> Self {
        return Self::UndeclaredVariable {
            varname: String::from(var_name),
            all_variables,
        };
    }

    pub fn type_mismtach(
        expected_ref: CodeReference,
        expected_type: VarType,
        found_ref: CodeReference,
        found_type: VarType,
    ) -> Self {
        CodeGenErrorKind::TypeMismatchInAssignment {
            expected_ref,
            expected_type,
            found_ref,
            found_type,
        }
    }
}

impl ParsingErrorKind {
    pub fn unexpected_tok(expected: &[Token], found: Token) -> Self {
        return Self::UnexpectedToken {
            expected: Box::from(expected),
            found,
        };
    }
}

impl Display for ParsingErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingErrorKind::UnexpectedEOF => {
                write!(f, "Unexpected EOF Error\n    Unexpected reach of EOF")
            }
            ParsingErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Unexpected Token Error\n    Expected one of the following tokens: "
                )?;
                for tok in expected.into_iter() {
                    write!(f, "\n\t {}", tok)?;
                }
                write!(f, "\n    Found:")?;
                write!(f, "\n\t {}", found)
            }
        }
    }
}

impl Display for CodeGenErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeGenErrorKind::UndeclaredVariable {
                varname,
                all_variables,
            } => {
                write!(f, "Undeclared Variable Error\n    Use of Undeclared Variable: {}\n    These are all of the declared variables: ", varname)?;
                // dbg!(all_variables);
                for var in all_variables.into_iter() {
                    write!(f, "{}, ", var)?;
                }
                write!(f, "\n    Fix this error by delacring the variable at the beginning of the program.")
            }
            CodeGenErrorKind::TypeMismatchInAssignment {
                expected_ref,
                expected_type,
                found_ref,
                found_type,
            } => {
                write!(f, "Provided Incorrect type in Assignment Error\n    Expected type {} because {} has type {}\n    But found {} with type {}\n    Fix this error by casting {} to {} using static_cast<{}>.",
                    expected_type, expected_ref, expected_type, found_ref, found_type, found_ref, expected_type, expected_type)
            }
        }
    }
}

impl Display for LexingErrorKind {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Display for CompilationErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError(err_desc) => write!(f, "  Internal error: {}", err_desc),
            Self::ParsingError(parsing_err) => write!(f, "  Parsing Error: {}", parsing_err),
            Self::LexingError(lexing_err) => write!(f, "  Lexing Error: {}", lexing_err),
            Self::CodeGenError(codegen_err) => {
                write!(f, "  Code Generation Error: {}", codegen_err)
            }
        }
    }
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n[Line {}, Column {}]:\n{}\n",
            self.line, self.column, self.err_kind
        )
    }
}

macro_rules! impl_debug_from_display {
    ($name:ty) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self, f)
            }
        }
    };
}

impl_debug_from_display!(CompilationError);
impl_debug_from_display!(CompilationErrorKind);
impl_debug_from_display!(LexingErrorKind);
impl_debug_from_display!(CodeGenErrorKind);
impl_debug_from_display!(ParsingErrorKind);
