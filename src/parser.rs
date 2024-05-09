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
    NotAList { line: usize, col: usize },
}
