use lime::lexer::{Lexer, Token};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Welcome to Lime interpreter!");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let mut lexer = Lexer::new(&line);
                loop {
                    match lexer.next() {
                        Ok(token) => match token {
                            Token::Start { line, col } => {
                                println!("Start(line: {}, col: {})", line, col)
                            }
                            Token::End { line, col } => {
                                println!("End(line: {}, col: {})", line, col)
                            }
                            Token::String { token, line, col } => {
                                println!("String(\"{}\", line: {}, col: {})", token, line, col)
                            }
                            Token::Number { token, line, col } => {
                                println!("Number({}, line: {}, col: {})", token, line, col)
                            }
                            Token::Symbol { token, line, col } => {
                                println!("Symbol({}, line: {}, col: {})", token, line, col)
                            }
                        },
                        Err(lime::lexer::LexerError::UnclosedString { line, col }) => {
                            println!("ERROR:{}:{}: Missing quote", line, col);
                            break;
                        }
                        Err(lime::lexer::LexerError::NoTokenFound) => break,
                    };
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Error: Keyboard Interrupt!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
