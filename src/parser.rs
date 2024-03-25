use crate::{
    codegen::{CodeGenerator, VarType},
    expression::Expression,
    lexer::Lexeme,
    token::{
        Keyword, Token, CAST_TOK, ID_TOK, INPUT_TOK, LPAREN_TOK, NUM_TOK, OUTPUT_TOK, RPAREN_TOK,
        SEMIC_TOK,
    },
};

#[derive(Default)]
struct Parser {
    generated_code: String,
    tokens: Vec<(Lexeme, Token)>,
    ptr: usize,
    code_generator: CodeGenerator,
}

impl Parser {
    fn lookahead(&self) -> Option<(Lexeme, Token)> {
        self.tokens.get(self.ptr).cloned()
    }

    fn lookahead_tok(&self) -> Option<Token> {
        self.tokens.get(self.ptr).map(|tok| tok.1)
    }

    fn lookahead_lexme(&self) -> Option<&Lexeme> {
        self.tokens.get(self.ptr).map(|tok| &tok.0)
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn match_tok(&mut self, tok: Token) -> Option<Lexeme> {
        let (lexeme, lookahead_tok) = self.lookahead()?;
        (lookahead_tok == tok).then(|| {
            self.advance();
            lexeme
        })
    }

    fn push_generated_code(&mut self, code: &str) {
        self.generated_code.push_str(code);
    }

    fn parse_id(&mut self) -> Option<Lexeme> {
        self.match_tok(ID_TOK)
    }

    fn parse_program(&mut self) -> Option<()> {
        self.parse_declerations()?;
        self.parse_stmt_block()?;
        Some(())
    }

    fn parse_declerations(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_stmt_block(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_type(&mut self) -> Option<VarType> {
        let lookahead = self.lookahead_tok()?;
        self.advance();
        match lookahead {
            Token::Keyword(Keyword::Int) => {
                return Some(VarType::Int);
            }
            Token::Keyword(Keyword::Float) => {
                return Some(VarType::Float);
            }
            _ => return None,
        }
    }

    fn parse_id_list(&mut self) -> Option<&[Lexeme]> {
        let mut id_list = Vec::new();

        while let Some(lexeme) = self.parse_id() {
            id_list.push(lexeme);
        }

        if id_list.is_empty() {
            return None;
        }

        Some(Vec::leak(id_list))
    }

    fn parse_input_statement(&mut self) -> Option<()> {
        self.match_tok(INPUT_TOK)?;
        self.match_tok(LPAREN_TOK)?;
        let var_name = self.parse_id()?.0;
        self.match_tok(RPAREN_TOK)?;
        self.match_tok(SEMIC_TOK)?;

        // Generate the code for the input statement
        let codegen = self.code_generator.gen_input_stmt(var_name)?; // CODEGEN
        self.push_generated_code(&codegen);

        Some(())
    }

    fn parse_output_statement(&mut self) -> Option<()> {
        todo!()
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_term(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let lookahead = self.lookahead_tok()?;
        match lookahead {
            CAST_TOK => {
                return self.parse_cast_expr();
            }
            ID_TOK => {
                return self.parse_id_expr();
            }
            NUM_TOK => {
                return self.parse_num_expr();
            }
            LPAREN_TOK => {
                let expr = self.parse_expression();
                self.match_tok(RPAREN_TOK)?;
                return expr;
            }
            _ => return None,
        }
    }

    fn parse_cast_expr(&mut self) -> Option<Expression> {
        let cast_lexeme = self.match_tok(CAST_TOK)?;
        let cast_type: VarType;
        if &*cast_lexeme.0 == "static_cast<int>" {
            cast_type = VarType::Int;
        } else if &*cast_lexeme.0 == "static_cast<float>" {
            cast_type = VarType::Float;
        }

        self.match_tok(LPAREN_TOK)?;
        let expr_to_cast = self.parse_expression()?;
        self.match_tok(RPAREN_TOK)?;

        todo!(); // CODEGEN

        return Some(Expression::cast(cast_type, expr_to_cast));
    }

    fn parse_id_expr(&mut self) -> Option<Expression> {
        let var_name = self.match_tok(ID_TOK)?.0;
        let var_type = self.code_generator.get_var_type(&var_name)?;
        Some(Expression::variable(var_name, var_type))
    }

    fn parse_num_expr(&mut self) -> Option<Expression> {
        let raw_num_str = self.match_tok(NUM_TOK)?.0;
        if raw_num_str.contains(".") {
            // Parse as a float
            //  TODO: don't panic, return an error
            let parsed_num: f32 = raw_num_str.parse().expect("Could not parse float literal.");
            return Some(Expression::float_literal(parsed_num));
        } else {
            // Parse as an int
            //  TODO: don't panic, return an error
            let parsed_num: i32 = raw_num_str.parse().expect("Could not parse int literal.");
            return Some(Expression::int_literal(parsed_num));
        }
    }
}