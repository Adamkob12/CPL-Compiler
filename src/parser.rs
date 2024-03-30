use crate::{
    boolexpr::*,
    codegen::{CodeGenerator, CodeReference, VarType},
    error::*,
    expression::{BinaryOp, Expression},
    lexer::{LexedToken, Lexeme},
    token::*,
};

#[derive(Default)]
pub struct Parser {
    generated_code: String,
    tokens: Vec<LexedToken>,
    ptr: usize,
    pub code_generator: CodeGenerator,
    last_seen_line: usize,
    last_seen_column: usize,
    errors_found: Vec<CompilationError>,
}

impl Parser {
    pub fn new(tokens: Vec<LexedToken>) -> Self {
        return Parser {
            generated_code: String::new(),
            tokens,
            ptr: 0,
            code_generator: CodeGenerator::new(),
            last_seen_line: 1,
            last_seen_column: 0,
            errors_found: Vec::new(),
        };
    }

    fn lookahead(&mut self) -> Result<LexedToken, CompilationError> {
        return self
            .tokens
            .get(self.ptr)
            .cloned()
            .ok_or_else(|| CompilationError::unexpected_eof())
            .inspect(|lexed_token| {
                self.last_seen_line = lexed_token.line;
                self.last_seen_column = lexed_token.column
            });
    }

    fn lookahead_tok(&mut self) -> Result<Token, CompilationError> {
        return self.lookahead().map(|lexed_token| lexed_token.token);
    }

    fn is_lookahead(&mut self, tok: Token) -> bool {
        return self
            .lookahead_tok()
            .map_or(false, |lookahead| tok == lookahead);
    }

    fn match_tok(&mut self, tok: Token) -> Result<Lexeme, CompilationError> {
        let lookahead = self.lookahead()?;
        return (lookahead.token == tok)
            .then(|| {
                self.ptr += 1;
                lookahead.lexeme
            })
            .ok_or(CompilationError::parsing_error(
                self.last_seen_line,
                self.last_seen_column,
                ParsingErrorKind::unexpected_tok(&[tok], lookahead.token),
            ));
    }

    fn push_generated_code(&mut self, code: &str) {
        self.generated_code.push_str(code);
    }

    fn cache_error<T>(&mut self, result: Result<T, CompilationError>) {
        if let Err(error) = result {
            self.errors_found.push(error);
        }
    }

    // ID
    fn parse_id(&mut self) -> Result<Lexeme, CompilationError> {
        return self.match_tok(ID_TOK);
    }

    /// declerations stmt_block
    pub fn parse_program(mut self) -> Result<String, Vec<CompilationError>> {
        let declerations = self.parse_declerations();
        self.cache_error(declerations);

        let stmt_block = self.parse_stmt_block();
        self.cache_error(stmt_block);

        self.push_generated_code("HALT");

        if self.errors_found.is_empty() {
            return Ok(self.generated_code);
        } else {
            return Err(self.errors_found);
        }
    }

    /// declerations decleration | epsilon
    fn parse_declerations(&mut self) -> Result<(), CompilationError> {
        if self.is_lookahead(ID_TOK) {
            self.parse_decleration()?;
            return self.parse_declerations();
        }
        return Ok(());
    }

    /// idlist : type ;
    fn parse_decleration(&mut self) -> Result<(), CompilationError> {
        let idlist = self.parse_id_list()?;
        self.match_tok(COLON_TOK)?;
        let ty = self.parse_type()?;
        for id in idlist.into_iter().cloned() {
            self.code_generator.register_variable(Box::leak(id.0), ty);
        }
        self.match_tok(SEMIC_TOK)?;
        return Ok(());
    }

    /// INT | FLOAT
    fn parse_type(&mut self) -> Result<VarType, CompilationError> {
        let lookahead_tok = self.lookahead_tok()?;
        match lookahead_tok {
            Token::Keyword(Keyword::Int) => {
                self.match_tok(INT_TOK)?;
                return Ok(VarType::Int);
            }
            Token::Keyword(Keyword::Float) => {
                self.match_tok(FLOAT_TOK)?;
                return Ok(VarType::Float);
            }
            _ => {}
        }

        return Err(CompilationError::parsing_error(
            self.last_seen_line,
            self.last_seen_column,
            ParsingErrorKind::unexpected_tok(&[INT_TOK, FLOAT_TOK], lookahead_tok),
        ));
    }

    /// idlist , ID | ID
    fn parse_id_list(&mut self) -> Result<Box<[Lexeme]>, CompilationError> {
        let mut id_list = Vec::new();
        id_list.push(self.parse_id()?);
        while let Ok(_) = self.match_tok(COMMA_TOK) {
            id_list.push(self.parse_id()?);
        }
        return Ok(id_list.into_boxed_slice());
    }

    /// INPUT ( ID ) ;
    fn parse_input_statement(&mut self) -> Result<(), CompilationError> {
        self.match_tok(INPUT_TOK)?;
        self.match_tok(LPAREN_TOK)?;
        let var_name = self.parse_id()?.0;
        self.match_tok(RPAREN_TOK)?;
        self.match_tok(SEMIC_TOK)?;
        // Generate the code for the input statement
        let generated_code =
            self.code_generator
                .gen_input_stmt(&var_name)
                .map_err(|codegen_err| {
                    CompilationError::codegen_error(
                        self.last_seen_line,
                        self.last_seen_column,
                        codegen_err,
                    )
                })?;
        self.push_generated_code(&generated_code);
        return Ok(());
    }

    /// OUTPUT ( expression ) ;
    fn parse_output_statement(&mut self) -> Result<(), CompilationError> {
        self.match_tok(OUTPUT_TOK)?;
        self.match_tok(LPAREN_TOK)?;
        let expr = self.parse_expression()?;
        self.match_tok(RPAREN_TOK)?;
        self.match_tok(SEMIC_TOK)?;
        let generated_code = self
            .code_generator
            .gen_output_stmt(expr)
            .map_err(|codegen_err| {
                CompilationError::codegen_error(
                    self.last_seen_line,
                    self.last_seen_column,
                    codegen_err,
                )
            })?;
        self.push_generated_code(&generated_code);
        return Ok(());
    }

    /// expression ADDOP term | term
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

    /// boolexpr OR boolterm | boolterm
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

    /// boolterm AND boolfactor | boolfactor
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

    /// term MULOP factor | factor
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

    /// expression RELOP expression | NOT ( boolexpr )
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

    /// ( expression ) | CAST ( expression ) | ID | NUM
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
                    self.last_seen_line,
                    self.last_seen_column,
                    ParsingErrorKind::unexpected_tok(
                        &[CAST_TOK, ID_TOK, NUM_TOK, LPAREN_TOK],
                        lookahead_tok,
                    ),
                ))
            }
        }
    }

    /// CAST ( expression )
    fn parse_cast_expr(&mut self) -> Result<Expression, CompilationError> {
        let cast_lexeme = self.match_tok(CAST_TOK)?;
        let cast_type: VarType;
        match &*cast_lexeme.0 {
            "static_cast<int>" => cast_type = VarType::Int,
            "static_cast<float>" => cast_type = VarType::Float,
            lexeme => {
                return Err(CompilationError::internal_error(format!(
                    "Lexer mistakeingly parsed {} as CAST token. Line {} Column {}",
                    lexeme, self.last_seen_line, self.last_seen_column
                )))
            }
        }

        self.match_tok(LPAREN_TOK)?;
        let expr_to_cast = self.parse_expression()?;
        self.match_tok(RPAREN_TOK)?;

        if expr_to_cast.ty == cast_type {
            return Ok(expr_to_cast);
        }

        return Ok(Expression::cast(
            cast_type,
            expr_to_cast,
            &mut self.code_generator,
        ));
    }

    /// ID (variable name)
    fn parse_id_expr(&mut self) -> Result<Expression, CompilationError> {
        let var_name = self.match_tok(ID_TOK)?.0;
        let var_type = self
            .code_generator
            .get_var_type(&var_name)
            .map_err(|codegen_err| {
                CompilationError::codegen_error(
                    self.last_seen_line,
                    self.last_seen_column,
                    codegen_err,
                )
            })?;
        return Ok(Expression::variable(var_name, var_type));
    }

    /// digit+(.digit+)?
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

    /// ID = expression ;
    fn parse_assignment_stmt(&mut self) -> Result<(), CompilationError> {
        let CodeReference::VarName(var_name) = self.parse_id_expr()?.code_ref else {
            return Err(CompilationError::internal_error(
                "Number was parsed as variable name".into(),
            ));
        };
        self.match_tok(EQ_TOK)?;
        let expr = self.parse_expression()?;
        self.match_tok(SEMIC_TOK)?;
        self.generated_code.push_str(
            &self
                .code_generator
                .gen_assignment_stmt(&var_name, expr)
                .map_err(|codegen_err| {
                    CompilationError::codegen_error(
                        self.last_seen_line,
                        self.last_seen_column,
                        codegen_err,
                    )
                })?,
        );
        return Ok(());
    }

    /// IF ( boolexpr ) stmt ELSE stmt
    // *boolexpr code* (assume the result is stored in variable r)
    // JMPZ L1 r
    // *stmt if boolexpr is true*
    // JUMP L2
    // L1: ("else label")
    // *stmt if boolexpr is false*
    // L2: ("post label")
    // *after if statement*
    fn parse_if_stmt(&mut self) -> Result<(), CompilationError> {
        self.match_tok(IF_TOK)?; // if
        self.match_tok(LPAREN_TOK)?; // (
        let boolexpr = self.parse_boolexpr()?; // boolexpr
        self.match_tok(RPAREN_TOK)?; // )

        let else_label = self.code_generator.new_label(); // request a new label for "else" from the code generator
        let post_label = self.code_generator.new_label(); // request a new label for "post" from the code generator

        self.push_generated_code(&boolexpr.code_generated); // boolexpr code
        self.push_generated_code(&self.code_generator.gen_jump_if_false(else_label, boolexpr)); // Jump to else if false
        self.parse_stmt()?;
        self.match_tok(ELSE_TOK)?;
        self.push_generated_code(&self.code_generator.gen_jump_to_label(post_label)); // Jump to post after stmt if true
        self.push_generated_code(&self.code_generator.gen_label_decleration(else_label)); // Declare else label
        self.parse_stmt()?;
        self.push_generated_code(&self.code_generator.gen_label_decleration(post_label)); // Declare post label

        return Ok(());
    }

    /// WHILE ( boolexpr ) stmt
    // L1:
    // *boolexpr code* (assume the result is stored in variable r)
    // JMPZ L2 r
    // *stmt code*
    // JUMP L1
    // L2:
    fn parse_while_stmt(&mut self) -> Result<(), CompilationError> {
        self.match_tok(WHILE_TOK)?; // while
        self.match_tok(LPAREN_TOK)?; // (
        let boolexpr = self.parse_boolexpr()?; // boolexpr
        self.match_tok(RPAREN_TOK)?; // )

        let loop_label = self.code_generator.new_label(); // request a new label for the loop from the code generator
        let break_label = self.code_generator.new_label(); // request a new label for breaking from the loop from the code generator

        self.push_generated_code(&self.code_generator.gen_label_decleration(loop_label)); // L1:
        self.push_generated_code(&boolexpr.code_generated); // code for the boolean expression
        self.push_generated_code(&self.code_generator.gen_jump_if_false(break_label, boolexpr)); // JMPZ L2 r
        self.parse_stmt()?;
        self.push_generated_code(&self.code_generator.gen_jump_to_label(loop_label)); // JUMP L1
        self.push_generated_code(&self.code_generator.gen_label_decleration(break_label)); // L2:

        return Ok(());
    }

    /// assignment_stmt | input_stmt | output_stmt | if_stmt | while_stmt | stmt_block
    fn parse_stmt(&mut self) -> Result<(), CompilationError> {
        let lookahead_tok = self.lookahead_tok()?;
        match lookahead_tok {
            ID_TOK => return self.parse_assignment_stmt(),
            INPUT_TOK => return self.parse_input_statement(),
            OUTPUT_TOK => return self.parse_output_statement(),
            WHILE_TOK => return self.parse_while_stmt(),
            IF_TOK => return self.parse_if_stmt(),
            LCURLY_TOK => return self.parse_stmt_block(),
            _ => {}
        }
        return Err(CompilationError::parsing_error(
            self.last_seen_line,
            self.last_seen_column,
            ParsingErrorKind::unexpected_tok(
                &[ID_TOK, INPUT_TOK, OUTPUT_TOK, IF_TOK, WHILE_TOK, LCURLY_TOK],
                lookahead_tok,
            ),
        ));
    }

    /// { stmtlist }
    fn parse_stmt_block(&mut self) -> Result<(), CompilationError> {
        self.match_tok(LCURLY_TOK)?;
        self.parse_stmtlist()?;
        self.match_tok(RCURLY_TOK)?;
        return Ok(());
    }

    /// stmt_list stmt | epsilon
    fn parse_stmtlist(&mut self) -> Result<(), CompilationError> {
        if self.is_lookahead(RCURLY_TOK) {
            return Ok(());
        }
        let stmt = self.parse_stmt();
        if let Err(error) = stmt {
            if let Some(ptr_to_next_stmt) = self.try_find_next_stmt() {
                self.ptr = ptr_to_next_stmt;
                self.errors_found.push(error);
            } else {
                return Err(error);
            }
        }
        return self.parse_stmtlist();
    }

    fn try_find_next_stmt(&self) -> Option<usize> {
        let mut ptr = self.ptr;
        while let Some(next_tok) = self.tokens.get(ptr) {
            if [ID_TOK, INPUT_TOK, OUTPUT_TOK, IF_TOK, WHILE_TOK, LCURLY_TOK]
                .contains(&next_tok.token)
            {
                return Some(ptr);
            }
            ptr += 1;
        }
        return None;
    }
}
