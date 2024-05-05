pub enum Token {
    Start {
        line: usize,
        col: usize,
    },
    End {
        line: usize,
        col: usize,
    },
    Symbol {
        token: String,
        line: usize,
        col: usize,
    },
    String {
        token: String,
        line: usize,
        col: usize,
    },
    Number {
        token: String,
        line: usize,
        col: usize,
    },
}

pub struct Lexer<'a> {
    input: &'a str,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            line: 1,
            col: 1,
        }
    }
}
