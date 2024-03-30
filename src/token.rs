#![allow(private_interfaces)]

use std::fmt::Display;
pub const REGEX_TABLE: &'static [(RegexMatch, &str)] = &[
    (RegexMatch::from_token_id(BREAK_ID), r"^break$"),
    (RegexMatch::from_token_id(CASE_ID), r"^case$"),
    (RegexMatch::from_token_id(DEFAULT_ID), r"^default$"),
    (RegexMatch::from_token_id(ELSE_ID), r"^else$"),
    (RegexMatch::from_token_id(FLOAT_ID), r"^float$"),
    (RegexMatch::from_token_id(IF_ID), r"^if$"),
    (RegexMatch::from_token_id(INPUT_ID), r"^input$"),
    (RegexMatch::from_token_id(INT_ID), r"^int$"),
    (RegexMatch::from_token_id(OUTPUT_ID), r"^output$"),
    (RegexMatch::from_token_id(SWITCH_ID), r"^switch$"),
    (RegexMatch::from_token_id(WHILE_ID), r"^while$"),
    (RegexMatch::from_token_id(RPAREN_ID), r"^\)$"),
    (RegexMatch::from_token_id(LPAREN_ID), r"^\($"),
    (RegexMatch::from_token_id(RCURLY_ID), r"^\}$"),
    (RegexMatch::from_token_id(LCURLY_ID), r"^\{$"),
    (RegexMatch::from_token_id(COMMA_ID), r"^,$"),
    (RegexMatch::from_token_id(COLON_ID), r"^:$"),
    (RegexMatch::from_token_id(SEMICOLON_ID), r"^;$"),
    (RegexMatch::from_token_id(EQUALS_ID), r"^=$"),
    (RegexMatch::from_token_id(RELOP_ID), r"^(==|!=|<|>|<=|>=)$"),
    (RegexMatch::from_token_id(ADDOP_ID), r"^(\+|-)$"),
    (RegexMatch::from_token_id(MULOP_ID), r"^(\*|/)$"),
    (RegexMatch::from_token_id(OR_ID), r"^\|\|$"),
    (RegexMatch::from_token_id(AND_ID), r"^&&$"),
    (RegexMatch::from_token_id(NOT_ID), r"^!$"),
    (
        RegexMatch::from_token_id(CAST_ID),
        r"^static_cast<(int|float)>$",
    ),
    (
        RegexMatch::from_token_id(IDENT_ID),
        r"^[a-zA-Z][_a-zA-Z0-9]*$",
    ),
    (RegexMatch::from_token_id(NUM_ID), r"^[0-9]+(\.[0-9]*)?$"),
    (RegexMatch::NonToken(NonToken::Spaces), r"^[ \t]+$"),
    (RegexMatch::NonToken(NonToken::StartComment), r"^/\*$"),
    (RegexMatch::NonToken(NonToken::EndComment), r"^\*/$"),
    (
        RegexMatch::NonToken(NonToken::Error(UNRECOGNIZED_TOKEN_ERR)),
        r"^.$",
    ),
    (
        RegexMatch::NonToken(NonToken::Error(UNRECOGNIZED_TOKEN_ERR)),
        r"^[0-9]+(.[0-9]*)?[a-zA-Z]+$",
    ),
];

pub fn build_regex_set() -> regex::RegexSet {
    regex::RegexSet::new(REGEX_TABLE.iter().map(|(_, regex)| regex))
        .expect("Failed to build regex set.")
}

const UNRECOGNIZED_TOKEN_ERR: &str = "Token Error: unrecognized token.";

pub type TokenID = u16;

pub enum RegexMatch {
    NonToken(NonToken),
    Token(Token),
}

const STARTING_TOKEN_ID: TokenID = 10;

impl RegexMatch {
    const fn from_token_id(id: TokenID) -> Self {
        RegexMatch::Token(TOKEN_TABLE[(id - STARTING_TOKEN_ID) as usize])
    }

    pub fn _get_id(&self) -> TokenID {
        match self {
            RegexMatch::NonToken(a) if matches!(a, NonToken::Spaces) => 0,
            RegexMatch::Token(token) => token.id(),
            _ => 1,
        }
    }
}

pub enum NonToken {
    Spaces,
    StartComment,
    EndComment,
    EOF,
    Error(&'static str),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Operator(Operator),
    Additional(Additional),
}

impl Token {
    pub fn id(&self) -> TokenID {
        match self {
            Token::Keyword(keyword) => *keyword as TokenID,
            Token::Symbol(symbol) => *symbol as TokenID,
            Token::Operator(operator) => *operator as TokenID,
            Token::Additional(additional) => *additional as TokenID,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{:?} (Keyword)", keyword),
            Token::Symbol(symbol) => write!(f, "{:?} (Symbol)", symbol),
            Token::Operator(operator) => write!(f, "{:?} (Operator)", operator),
            Token::Additional(additional) => write!(f, "{:?}", additional),
        }
    }
}

const TOKEN_TABLE: &'static [Token] = &[
    Token::Keyword(Keyword::Break),
    Token::Keyword(Keyword::Case),
    Token::Keyword(Keyword::Default),
    Token::Keyword(Keyword::Else),
    Token::Keyword(Keyword::Float),
    Token::Keyword(Keyword::If),
    Token::Keyword(Keyword::Input),
    Token::Keyword(Keyword::Int),
    Token::Keyword(Keyword::Output),
    Token::Keyword(Keyword::Switch),
    Token::Keyword(Keyword::While),
    Token::Symbol(Symbol::RParen),
    Token::Symbol(Symbol::LParen),
    Token::Symbol(Symbol::RCurly),
    Token::Symbol(Symbol::LCurly),
    Token::Symbol(Symbol::Comma),
    Token::Symbol(Symbol::Colon),
    Token::Symbol(Symbol::SemiColon),
    Token::Symbol(Symbol::Equals),
    Token::Operator(Operator::RELOP),
    Token::Operator(Operator::ADDOP),
    Token::Operator(Operator::MULOP),
    Token::Operator(Operator::OR),
    Token::Operator(Operator::AND),
    Token::Operator(Operator::NOT),
    Token::Operator(Operator::CAST),
    Token::Additional(Additional::Ident),
    Token::Additional(Additional::Num),
];

pub const BREAK_ID: TokenID = 10;
pub const CASE_ID: TokenID = 11;
pub const DEFAULT_ID: TokenID = 12;
pub const ELSE_ID: TokenID = 13;
pub const FLOAT_ID: TokenID = 14;
pub const IF_ID: TokenID = 15;
pub const INPUT_ID: TokenID = 16;
pub const INT_ID: TokenID = 17;
pub const OUTPUT_ID: TokenID = 18;
pub const SWITCH_ID: TokenID = 19;
pub const WHILE_ID: TokenID = 20;
pub const RPAREN_ID: TokenID = 21;
pub const LPAREN_ID: TokenID = 22;
pub const RCURLY_ID: TokenID = 23;
pub const LCURLY_ID: TokenID = 24;
pub const COMMA_ID: TokenID = 25;
pub const COLON_ID: TokenID = 26;
pub const SEMICOLON_ID: TokenID = 27;
pub const EQUALS_ID: TokenID = 28;
pub const RELOP_ID: TokenID = 29;
pub const ADDOP_ID: TokenID = 30;
pub const MULOP_ID: TokenID = 31;
pub const OR_ID: TokenID = 32;
pub const AND_ID: TokenID = 33;
pub const NOT_ID: TokenID = 34;
pub const CAST_ID: TokenID = 35;
pub const IDENT_ID: TokenID = 36;
pub const NUM_ID: TokenID = 37;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Keyword {
    Break = BREAK_ID,
    Case = CASE_ID,
    Default = DEFAULT_ID,
    Else = ELSE_ID,
    Float = FLOAT_ID,
    If = IF_ID,
    Input = INPUT_ID,
    Int = INT_ID,
    Output = OUTPUT_ID,
    Switch = SWITCH_ID,
    While = WHILE_ID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Symbol {
    RParen = RPAREN_ID,
    LParen = LPAREN_ID,
    RCurly = RCURLY_ID,
    LCurly = LCURLY_ID,
    Comma = COMMA_ID,
    Colon = COLON_ID,
    SemiColon = SEMICOLON_ID,
    Equals = EQUALS_ID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Operator {
    RELOP = RELOP_ID,
    ADDOP = ADDOP_ID,
    MULOP = MULOP_ID,
    OR = OR_ID,
    AND = AND_ID,
    NOT = NOT_ID,
    CAST = CAST_ID,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Additional {
    Ident = IDENT_ID,
    Num = NUM_ID,
}

// KEYWORDS
pub const INPUT_TOK: Token = Token::Keyword(Keyword::Input);
pub const OUTPUT_TOK: Token = Token::Keyword(Keyword::Output);
pub const INT_TOK: Token = Token::Keyword(Keyword::Int);
pub const FLOAT_TOK: Token = Token::Keyword(Keyword::Float);
pub const IF_TOK: Token = Token::Keyword(Keyword::If);
pub const ELSE_TOK: Token = Token::Keyword(Keyword::Else);
pub const WHILE_TOK: Token = Token::Keyword(Keyword::While);
// OPERATOR
pub const CAST_TOK: Token = Token::Operator(Operator::CAST);
pub const MULOP_TOK: Token = Token::Operator(Operator::MULOP);
pub const AND_TOK: Token = Token::Operator(Operator::AND);
pub const OR_TOK: Token = Token::Operator(Operator::OR);
pub const ADDOP_TOK: Token = Token::Operator(Operator::ADDOP);
pub const NOT_TOK: Token = Token::Operator(Operator::NOT);
pub const RELOP_TOK: Token = Token::Operator(Operator::RELOP);
// SYMBOLS
pub const RPAREN_TOK: Token = Token::Symbol(Symbol::RParen);
pub const LPAREN_TOK: Token = Token::Symbol(Symbol::LParen);
pub const RCURLY_TOK: Token = Token::Symbol(Symbol::RCurly);
pub const LCURLY_TOK: Token = Token::Symbol(Symbol::LCurly);
pub const COLON_TOK: Token = Token::Symbol(Symbol::Colon);
pub const COMMA_TOK: Token = Token::Symbol(Symbol::Comma);
pub const SEMIC_TOK: Token = Token::Symbol(Symbol::SemiColon);
pub const EQ_TOK: Token = Token::Symbol(Symbol::Equals);
// ADDITIONAL
pub const ID_TOK: Token = Token::Additional(Additional::Ident);
pub const NUM_TOK: Token = Token::Additional(Additional::Num);
