mod pager;
mod parser;
mod table;
#[cfg(test)]
mod tests;

use anyhow::{bail, Result};

use crate::pager::Pager;
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

fn prepare_sql(line: &str) -> Result<SQL> {
    let sql = parse_sql(line)?;
    if let SQL::Insert(ref row) = sql {
        row.validate()?;
    }
    Ok(sql)
}

fn execute_sql<'a>(line: &str, table: &'a mut Table<'a>) -> Result<()> {
    let sql = prepare_sql(line)?;
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

fn process<'a>(line: &str, table: &'a mut Table<'a>) -> Result<()> {
    if let Some(char) = line.chars().nth(0) {
        if char == '.' {
            return execute_command(line);
        }
    }
    execute_sql(line, table)
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 1 {
        bail!("Must supply a db filename.");
    }
    let mut pager = Pager::new(&args[0])?;
    let mut table = Table::new(&mut pager);

    let mut rl = Editor::<()>::new()?;
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
