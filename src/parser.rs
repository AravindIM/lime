use core::fmt;
use std::fmt::Formatter;

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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnclosedString { line, col } => {
                return write!(f, "ERROR:{}:{}: Missing closing for string!", line, col);
            }
            Self::UnclosedList { line, col } => {
                return write!(f, "ERROR:{}:{}: Missing closing for list!", line, col);
            }
            Self::ExtraneousClosingList { line, col } => {
                return write!(
                    f,
                    "ERROR:{}:{}: Extraneous closing for string found",
                    line, col
                );
            }
        }
    }
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
