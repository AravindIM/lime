use core::fmt;
use std::fmt::Formatter;

use crate::lexer::{Lexer, LexerError, Token};

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
                    "ERROR:{}:{}: Extraneous closing for list found",
                    line, col
                );
            }
        }
    }
}

pub fn parse<'a>(lexer: &'a mut Lexer) -> Result<Vec<AstNode<'a>>, ParserError> {
    let mut program: Vec<AstNode> = vec![];
    let mut list_stack: Vec<AstNode> = vec![];
    loop {
        match lexer.next() {
            Ok(token) => match token {
                Token::Start { line, col } => {
                    let new_list = AstNode::List {
                        elements: vec![],
                        line,
                        col,
                    };
                    list_stack.push(new_list);
                }
                Token::End { line, col } => match list_stack.pop() {
                    Some(current_list) => match list_stack.last_mut() {
                        Some(previous_list) => {
                            if let AstNode::List {
                                ref mut elements, ..
                            } = *previous_list
                            {
                                elements.push(Box::new(current_list));
                            }
                        }
                        None => {
                            program.push(current_list);
                        }
                    },
                    None => return Err(ParserError::ExtraneousClosingList { line, col }),
                },
                Token::Number { token, line, col } => {
                    let node = AstNode::Number {
                        value: token,
                        line,
                        col,
                    };
                    match list_stack.last_mut() {
                        Some(previous_list) => {
                            if let AstNode::List {
                                ref mut elements, ..
                            } = *previous_list
                            {
                                elements.push(Box::new(node));
                            }
                        }
                        None => program.push(node),
                    }
                }
                Token::String { token, line, col } => {
                    let node = AstNode::String {
                        value: token,
                        line,
                        col,
                    };
                    match list_stack.last_mut() {
                        Some(previous_list) => {
                            if let AstNode::List {
                                ref mut elements, ..
                            } = *previous_list
                            {
                                elements.push(Box::new(node));
                            }
                        }
                        None => program.push(node),
                    }
                }
                _ => {}
            },
            Err(error) => match error {
                LexerError::UnclosedString { line, col } => {
                    return Err(ParserError::UnclosedString { line, col });
                }
                LexerError::NoTokenFound => {
                    if list_stack.is_empty() {
                        break;
                    } else {
                        return Err(ParserError::UnclosedList {
                            line: lexer.line,
                            col: lexer.col,
                        });
                    }
                }
            },
        }
    }
    return Ok(program);
}
