const CARRIAGE_RETURN: &str = "\r";
const WHITESPACE: &str = " ";
const TAB: &str = "\t";
const LINE_FEED: &str = "\n";
const START: &str = "(";
const END: &str = ")";
const STRING_QUOTE: &str = "\"";
const DECIMAL_POINT: &str = ".";

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
        let mut has_num_check_failed: bool = false;

        'lexer_loop: while self.input.len() > 0 {
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
                START => {
                    let token = Token::Start {
                        line: self.line,
                        col: self.col,
                    };
                    self.advance_column(1);
                    return Ok(token);
                }
                END => {
                    let token = Token::Start {
                        line: self.line,
                        col: self.col,
                    };
                    self.advance_column(1);
                    return Ok(token);
                }
                STRING_QUOTE => {
                    for i in 1..self.input.len() {
                        if &self.input[i..i + 1] == STRING_QUOTE {
                            let token = Token::String {
                                token: self.input[1..i].to_owned(),
                                line: self.line,
                                col: self.col,
                            };
                            self.advance_column(i + 1);
                            return Ok(token);
                        } else if &self.input[i..i + 1] == LINE_FEED {
                            return Err(LexerError::UnclosedString {
                                line: self.line,
                                col: self.col,
                            });
                        }
                    }
                    self.advance_column(self.input.len());
                    return Err(LexerError::UnclosedString {
                        line: self.line,
                        col: self.col,
                    });
                }
                number
                    if (number.as_bytes()[0].is_ascii_digit()
                        || &number[0..1] == DECIMAL_POINT)
                        && !has_num_check_failed =>
                {
                    let mut has_decimal_point = false;
                    let mut i = 0;
                    while i < self.input.len() {
                        match &self.input[i..i + 1] {
                            y if y.as_bytes()[0].is_ascii_digit() => {}
                            DECIMAL_POINT => {
                                if has_decimal_point {
                                    has_num_check_failed = true;
                                    continue 'lexer_loop;
                                } else {
                                    has_decimal_point = true;
                                }
                            }
                            WHITESPACE | LINE_FEED | CARRIAGE_RETURN | START | END => break,
                            _ => {
                                has_num_check_failed = true;
                                continue 'lexer_loop;
                            }
                        }
                        i += 1;
                    }
                    let token = Token::Number {
                        token: self.input[0..i].to_owned(),
                        line: self.line,
                        col: self.col,
                    };
                    self.advance_column(i);
                    return Ok(token);
                }
                _ => {
                    let mut i = 0;
                    while i < self.input.len() {
                        match &self.input[i..i + 1] {
                            WHITESPACE | LINE_FEED | CARRIAGE_RETURN | START | END => break,
                            _ => {}
                        }
                        i += 1;
                    }
                    let token = Token::Symbol {
                        token: self.input[0..i].to_owned(),
                        line: self.line,
                        col: self.col,
                    };
                    self.advance_column(i);
                    return Ok(token);
                }
            }
        }
        return Err(LexerError::NoTokenFound);
    }
}
