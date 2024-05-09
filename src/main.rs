use lime::lexer::Lexer;
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
                        Ok(token) => println!("{:?}", token),
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
