const CARRIAGE_RETURN: &str = "\r";
const WHITESPACE: &str = " ";
const TAB: &str = "\t";
const LINE_FEED: &str = "\n";

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

pub enum LexerError {
    NoTokenFound,
    UnclosedString { line: usize, col: usize },
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

    fn skip(&mut self, delta: usize) {
        self.input = &self.input[delta..];
    }

    fn advance_column(&mut self, delta: usize) {
        self.col += delta;
        self.skip(delta);
    }

    fn advance_line(&mut self, delta: usize) {
        self.col = 0;
        self.line += delta;
        self.skip(delta);
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        while self.input.len() > 0 {
            match &self.input[0..1] {
                CARRIAGE_RETURN => {
                    while &self.input[0..1] == CARRIAGE_RETURN {
                        self.skip(1);
                    }
                }
                WHITESPACE | TAB => {
                    while &self.input[0..1] == WHITESPACE || &self.input[0..1] == TAB {
                        self.advance_column(1);
                    }
                }
                LINE_FEED => {
                    while &self.input[0..1] == LINE_FEED {
                        self.advance_line(1);
                    }
                }
                _ => {}
            }
        }
        return Err(LexerError::NoTokenFound);
    }
}
