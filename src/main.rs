use lime::lexer::Lexer;
use lime::parser::parse;
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
                match parse(&mut lexer) {
                    Ok(ast) => {
                        dbg!(ast);
                    }
                    Err(e) => println!("{}", e),
                }
                // loop {
                //     match lexer.next() {
                //         Ok(token) => println!("{:?}", token),
                //         Err(e) => {
                //             println!("{}", e);
                //             break;
                //         }
                //     };
                // }
            }
            Err(ReadlineError::Interrupted) => {
                println!("ERROR: Keyboard Interrupt!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                println!("ERROR: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
