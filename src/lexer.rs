use crate::mfl_regex;

pub enum Token<'a> {
    Comment,
    Let,
    If,
    Else,
    Fn,
    While,
    Identifier(&'a str),
    Whitespace,
    Number,
    Add,
    Multiply,
    Subtract,
    Divide,
    Modulo,
    LessThanEqual,
    LessThan,
    GreaterThanEqual,
    GreaterThan,
    Equal,
    NotEqual,
    Not,
    And,
    Or,
    Assign,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    Semicolon,
    Colon,
    Arrow
}

pub struct TokenInfo<'a> {
    token: Token<'a>,
    start_pos: usize
}

pub fn tokenise<'a>(source: &'a str) -> Vec<TokenInfo<'a>> {
    mfl_regex::TOKEN.captures_iter(source).map(|m| {
        println!("{}",m.len());
        TokenInfo {
            token:Token::Add,
            start_pos: 0
        }
    }).collect()
}