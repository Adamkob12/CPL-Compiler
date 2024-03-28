use crate::{
    boolexpr::*,
    codegen::{CodeGenerator, VarType},
    expression::{BinaryOp, Expression},
    lexer::{LexedToken, Lexeme},
    token::{
        Keyword, Token, ADDOP_TOK, AND_TOK, CAST_TOK, FLOAT_TOK, ID_TOK, INPUT_TOK, INT_TOK,
        LPAREN_TOK, MULOP_TOK, NOT_TOK, NUM_TOK, OR_TOK, RELOP_TOK, RPAREN_TOK, SEMIC_TOK,
    },
};

mod error;
pub use error::*;

#[derive(Default)]
pub struct Parser {
    generated_code: String,
    tokens: Vec<LexedToken>,
    ptr: usize,
    pub code_generator: CodeGenerator,
    last_line: usize,
    last_column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<LexedToken>) -> Self {
        Parser {
            generated_code: String::new(),
            tokens,
            ptr: 0,
            code_generator: CodeGenerator::new(),
            last_line: 0,
            last_column: 0,
        }
    }

    fn lookahead(&self) -> Result<LexedToken, CompilationError> {
        self.tokens
            .get(self.ptr)
            .cloned()
            .ok_or_else(|| CompilationError::unexpected_eof())
    }

    fn lookahead_tok(&self) -> Result<Token, CompilationError> {
        self.tokens
            .get(self.ptr)
            .map(|tok| tok.token)
            .ok_or(CompilationError::unexpected_eof())
    }

    fn lookahead_lexme(&self) -> Result<&Lexeme, CompilationError> {
        self.tokens
            .get(self.ptr)
            .map(|tok| &tok.lexeme)
            .ok_or(CompilationError::unexpected_eof())
    }

    fn is_lookahead(&self, tok: Token) -> bool {
        return self
            .lookahead_tok()
            .map_or(false, |lookahead| tok == lookahead);
    }

    fn match_tok(&mut self, tok: Token) -> Result<Lexeme, CompilationError> {
        let lookahead = self.lookahead()?;
        (lookahead.token == tok)
            .then(|| {
                self.ptr += 1;
                self.last_line = lookahead.line;
                self.last_column = lookahead.column;
                lookahead.lexeme
            })
            .ok_or(CompilationError::parsing_error(
                self.last_line,
                self.last_column,
                ParsingErrorKind::unexpected_tok(&[tok], lookahead.token),
            ))
    }

    fn push_generated_code(&mut self, code: &str) {
        self.generated_code.push_str(code);
    }

    fn parse_id(&mut self) -> Result<Lexeme, CompilationError> {
        self.match_tok(ID_TOK)
    }

    fn parse_program(&mut self) -> Result<(), CompilationError> {
        self.parse_declerations()?;
        self.parse_stmt_block()?;
        Ok(())
    }

    fn parse_declerations(&mut self) -> Result<(), CompilationError> {
        todo!()
    }

    fn parse_stmt_block(&mut self) -> Result<(), CompilationError> {
        todo!()
    }

    fn parse_type(&mut self) -> Result<VarType, CompilationError> {
        let lookahead = self.lookahead_tok()?;
        match lookahead {
            Token::Keyword(Keyword::Int) => {
                self.match_tok(INT_TOK)?;
                return Ok(VarType::Int);
            }
            Token::Keyword(Keyword::Float) => {
                self.match_tok(FLOAT_TOK)?;
                return Ok(VarType::Float);
            }
            tok => {
                return Err(CompilationError::parsing_error(
                    self.last_line,
                    self.last_column,
                    ParsingErrorKind::unexpected_tok(&[INT_TOK, FLOAT_TOK], tok),
                ))
            }
        }
    }

    fn parse_id_list(&mut self) -> Option<&[Lexeme]> {
        let mut id_list = Vec::new();

        while let Ok(lexeme) = self.parse_id() {
            id_list.push(lexeme);
        }

        if id_list.is_empty() {
            return None;
        }

        Some(Vec::leak(id_list))
    }

    fn parse_input_statement(&mut self) -> Result<(), CompilationError> {
        self.match_tok(INPUT_TOK)?;
        self.match_tok(LPAREN_TOK)?;
        let var_name = self.parse_id()?.0;
        self.match_tok(RPAREN_TOK)?;
        self.match_tok(SEMIC_TOK)?;

        // Generate the code for the input statement
        let codegen = self
            .code_generator
            .gen_input_stmt(var_name)
            .map_err(|codegen_err| {
                CompilationError::codegen_error(self.last_line, self.last_column, codegen_err)
            })?;
        self.push_generated_code(&codegen);

        Ok(())
    }

    fn parse_output_statement(&mut self) -> Option<()> {
        todo!()
    }

    pub fn parse_expression(&mut self) -> Result<Expression, CompilationError> {
        let term = self.parse_term()?;
        if let Ok(addop) = self.match_tok(ADDOP_TOK) {
            let binop = BinaryOp::from_lexeme(addop);
            return Ok(Expression::binary_op(
                term,
                self.parse_expression()?,
                binop,
                &mut self.code_generator,
            ));
        }

        return Ok(term);
    }

    pub fn parse_boolexpr(&mut self) -> Result<BoolExpr, CompilationError> {
        let term = self.parse_boolterm()?;
        if let Ok(..) = self.match_tok(OR_TOK) {
            return Ok(BoolExpr::or(
                term,
                self.parse_boolexpr()?,
                &mut self.code_generator,
            ));
        }

        return Ok(term);
    }

    fn parse_boolterm(&mut self) -> Result<BoolExpr, CompilationError> {
        let factor = self.parse_boolfactor()?;
        if let Ok(..) = self.match_tok(AND_TOK) {
            return Ok(BoolExpr::and(
                factor,
                self.parse_boolterm()?,
                &mut self.code_generator,
            ));
        }

        return Ok(factor);
    }

    fn parse_term(&mut self) -> Result<Expression, CompilationError> {
        let factor = self.parse_factor()?;
        if let Ok(mulop) = self.match_tok(MULOP_TOK) {
            let binop = BinaryOp::from_lexeme(mulop);
            return Ok(Expression::binary_op(
                factor,
                self.parse_term()?,
                binop,
                &mut self.code_generator,
            ));
        }

        return Ok(factor);
    }

    fn parse_boolfactor(&mut self) -> Result<BoolExpr, CompilationError> {
        let lookahead = self.lookahead_tok()?;
        match lookahead {
            NOT_TOK => {
                self.match_tok(NOT_TOK)?;
                self.match_tok(LPAREN_TOK)?;
                let bool_expr = self.parse_boolexpr()?;
                self.match_tok(RPAREN_TOK)?;
                return Ok(bool_expr);
            }
            _ => (),
        }

        let expr1 = self.parse_expression()?;
        let relop_lexeme = self.match_tok(RELOP_TOK)?;
        let expr2 = self.parse_expression()?;

        return Ok(BoolExpr::relop(
            expr1,
            expr2,
            RelOp::from_lexeme(relop_lexeme),
            &mut self.code_generator,
        ));
    }

    fn parse_factor(&mut self) -> Result<Expression, CompilationError> {
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
                self.match_tok(LPAREN_TOK)?;
                let expr = self.parse_expression();
                self.match_tok(RPAREN_TOK)?;
                return expr;
            }
            lookahead_tok => {
                return Err(CompilationError::parsing_error(
                    self.last_line,
                    self.last_column,
                    ParsingErrorKind::unexpected_tok(
                        &[CAST_TOK, ID_TOK, NUM_TOK, LPAREN_TOK],
                        lookahead_tok,
                    ),
                ))
            }
        }
    }

    fn parse_cast_expr(&mut self) -> Result<Expression, CompilationError> {
        let cast_lexeme = self.match_tok(CAST_TOK)?;
        let cast_type: VarType;
        match &*cast_lexeme.0 {
            "static_cast<int>" => cast_type = VarType::Int,
            "static_cast<float>" => cast_type = VarType::Float,
            lexeme => {
                return Err(CompilationError::internal_error(format!(
                    "Lexer mistakeingly parsed {} as CAST token. Line {} Column {}",
                    lexeme, self.last_line, self.last_column
                )))
            }
        }

        self.match_tok(LPAREN_TOK)?;
        let expr_to_cast = self.parse_expression()?;
        self.match_tok(RPAREN_TOK)?;

        return Ok(Expression::cast(
            cast_type,
            expr_to_cast,
            &mut self.code_generator,
        ));
    }

    fn parse_id_expr(&mut self) -> Result<Expression, CompilationError> {
        let var_name = self.match_tok(ID_TOK)?.0;
        let var_type = self
            .code_generator
            .get_var_type(&var_name)
            .map_err(|codegen_err| {
                CompilationError::codegen_error(self.last_line, self.last_column, codegen_err)
            })?;
        Ok(Expression::variable(var_name, var_type))
    }

    fn parse_num_expr(&mut self) -> Result<Expression, CompilationError> {
        let raw_num_str = self.match_tok(NUM_TOK)?.0;
        let raw_num_str = raw_num_str.trim();
        if raw_num_str.contains(".") {
            // Parse as a float
            //  TODO: don't panic, return an error
            let parsed_num: f32 = raw_num_str
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse float literal: {}.", raw_num_str));
            return Ok(Expression::float_literal(parsed_num));
        } else {
            // Parse as an int
            //  TODO: don't panic, return an error
            let parsed_num: i32 = raw_num_str
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse int literal: {}.", raw_num_str));
            return Ok(Expression::int_literal(parsed_num));
        }
    }
}
