mod parser;
mod table;
#[cfg(test)]
mod tests;

use anyhow::Result;

use crate::parser::{parse_sql, SQL};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use table::Table;

fn execute_command(line: &str) -> Result<()> {
    match line {
        ".exit" => {
            println!("Exiting.");
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command: {}", line);
            Ok(())
        }
    }
}

fn execute_sql(line: &str, table: &mut Table) -> Result<()> {
    let sql = parse_sql(line)?;
    match sql {
        SQL::Select => {
            for row in table.select() {
                println!("{}", row);
            }
            Ok(())
        }
        SQL::Insert(row) => {
            table.insert(row)?;
            println!("Executed.");
            Ok(())
        }
    }
}

fn process(line: &str, table: &mut Table) -> Result<()> {
    if let Some(char) = line.chars().nth(0) {
        if char == '.' {
            return execute_command(line);
        }
    }
    execute_sql(line, table)
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let mut table = Table::new();
    loop {
        let readline = rl.readline("database > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if let Err(e) = process(&line, &mut table) {
                    println!("Error: {:?}", e);
                }
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
