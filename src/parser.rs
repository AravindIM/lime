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
