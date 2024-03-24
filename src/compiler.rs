use crate::lexer::Lexer;

pub struct Compiler {
    source_code: String,
}

impl Compiler {
    pub fn init(source_code: String) -> Compiler {
        Compiler { source_code }
    }

    // Compile the source code, output a string
    pub fn compile(&self) -> String {
        let mut lexer = Lexer::new(&self.source_code);

        todo!()
    }
}
