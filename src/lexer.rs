use crate::token::*;

#[repr(transparent)]
pub struct Lexeme(pub Box<str>);

impl<'a> Clone for Lexeme {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Clone)]
pub struct LexedToken {
    pub lexeme: Lexeme,
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

// A struct that represents the Lexer
pub struct Lexer {
    lines: Vec<String>,
    current_line: usize,
    current_char: usize,
    in_comment: bool,
    regex_set: regex::RegexSet,
}

impl Lexer {
    pub fn lex_tokens(source_code: String) -> Vec<LexedToken> {
        return Lexer::new(source_code).get_all_tokens();
    }

    pub fn new(source_code: String) -> Lexer {
        return Lexer {
            lines: source_code.lines().map(|slc| String::from(slc)).collect(),
            current_line: 0,
            current_char: 0,
            in_comment: false,
            regex_set: build_regex_set(),
        };
    }

    fn new_line(&mut self) {
        self.current_line += 1;
        self.current_char = 0;
    }

    // The main function of the Lexer
    pub fn get_next_token(&mut self) -> Option<LexedToken> {
        if self.current_line >= self.lines.len() {
            return None;
        }
        if self.current_char >= self.lines[self.current_line].len() {
            self.new_line();
            return self.get_next_token();
        }

        let mut line = &self.lines[self.current_line][self.current_char..];

        let mut regex_matches = self.regex_set.matches(line);
        while !regex_matches.matched_any() {
            line = &line[..(line.len() - 1)];
            regex_matches = self.regex_set.matches(line);
        }

        let match_index = regex_matches.iter().next().unwrap();
        let matched = &REGEX_TABLE[match_index].0;

        self.current_char += line.len();
        match matched {
            RegexMatch::NonToken(non_token) => match non_token {
                NonToken::StartComment => {
                    self.in_comment = true;
                }
                NonToken::EndComment => {
                    self.in_comment = false;
                }
                NonToken::Error(err) if !self.in_comment => {
                    eprintln!("Error at line {}: {}", self.current_line, err);
                    self.new_line();
                }
                _ => {}
            },
            RegexMatch::Token(token) if !self.in_comment => {
                return Some(LexedToken {
                    lexeme: Lexeme(line.into()),
                    token: *token,
                    line: self.current_line + 1,
                    column: self.current_char,
                });
            }
            _ => {}
        }

        // If we didn't return a token, then we need to get the next one.
        return self.get_next_token();
    }

    // Lex all the tokens at once
    pub fn get_all_tokens(mut self) -> Vec<LexedToken> {
        let mut toks = Vec::new();
        while let Some(tok) = self.get_next_token() {
            toks.push(tok);
        }
        return toks;
    }
}
