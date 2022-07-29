use anyhow::Result;
use rustyline::{Editor};
use rustyline::error::ReadlineError;

fn process(line: &str) -> Result<()> {
    match line {
        ".exit" => {
            std::process::exit(0);
        },
        _ => {
            println!("Unrecognized command: {}", line)
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    loop {
        let readline = rl.readline("database > ");
        match readline {
            Ok(line) => {
                process(&line)?;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
