use crate::{lexer::Lexer, parser::Parser};

pub struct Compiler {
    source_code: String,
}

impl Compiler {
    pub fn init(source_code: String) -> Compiler {
        return Compiler { source_code };
    }

    // Compile the source code, output a string
    pub fn compile(self) -> Option<String> {
        let parser = Parser::new(Lexer::lex_tokens(self.source_code));
        return parser.parse_program().map_or_else(
            |errors| {
                for error in errors {
                    eprintln!("{}", error);
                }
                None
            },
            |output| Some(output),
        );
    }
}
