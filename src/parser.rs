use crate::lexer::{Lexer, LexerError};

pub enum AstNode<'a> {
    List {
        elements: Vec<Box<AstNode<'a>>>,
        line: usize,
        col: usize,
    },
    String {
        value: &'a str,
        line: usize,
        col: usize,
    },
    Number {
        value: &'a str,
        line: usize,
        col: usize,
    },
    Symbol {
        value: &'a str,
        line: usize,
        col: usize,
    },
}

pub enum ParserError {
    UnclosedString { line: usize, col: usize },
    UnclosedList { line: usize, col: usize },
    ExtraneousClosingList { line: usize, col: usize },
}

pub fn parse<'a>(lexer: &'a mut Lexer) -> Result<Vec<AstNode<'a>>, ParserError> {
    let mut program: Vec<AstNode> = vec![];
    loop {
        match lexer.next() {
            Ok(token) => {}
            Err(error) => match error {
                LexerError::UnclosedString { line, col } => {
                    return Err(ParserError::UnclosedList { line, col });
                }
                LexerError::NoTokenFound => break,
            },
        }
    }
    return Ok(program);
}
